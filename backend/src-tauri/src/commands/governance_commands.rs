use std::path::Path;

use tauri::State;

use crate::domain::governance::{
    GovernanceAnalysis, GovernanceScanResult, Recommendation, RecommendationStatus,
};
use crate::domain::governance_analysis::{
    build_analysis_prompt, build_recommendations, current_timestamp, send_and_collect,
    write_recommendation_file, GOVERNANCE_SYSTEM_PROMPT,
};
use crate::domain::governance_scanner::scan_governance;
use crate::error::OrqaError;
use crate::repo::{governance_repo, project_repo, session_repo};
use crate::state::AppState;

/// Scan a project's filesystem for governance files.
///
/// Looks up the project's path from the database, then walks the filesystem
/// collecting governance files from all known tool ecosystems.
#[tauri::command]
pub fn governance_scan(
    project_id: i64,
    state: State<'_, AppState>,
) -> Result<GovernanceScanResult, OrqaError> {
    let project_path = get_project_path(project_id, &state)?;
    scan_governance(Path::new(&project_path))
}

/// Analyze governance coverage using Claude and persist the results.
///
/// Creates a dedicated governance session, sends the scan data to Claude as a
/// structured prompt, parses Claude's JSON response, and saves the analysis
/// and recommendations to the database.
#[tauri::command]
pub fn governance_analyze(
    project_id: i64,
    scan_result: GovernanceScanResult,
    state: State<'_, AppState>,
) -> Result<GovernanceAnalysis, OrqaError> {
    let session_id = create_governance_session(project_id, &state)?;
    let prompt = build_analysis_prompt(&scan_result);
    super::sidecar_commands::ensure_sidecar_running(&state)?;
    let raw_response = send_and_collect(&state.sidecar.manager, session_id, &prompt)?;
    let output = crate::domain::governance_analysis::parse_claude_output(&raw_response)?;
    let now = current_timestamp();
    let analysis = GovernanceAnalysis {
        id: 0,
        project_id,
        scan_data: scan_result,
        summary: output.summary.clone(),
        strengths: output.strengths.clone(),
        gaps: output.gaps.clone(),
        session_id: Some(session_id),
        created_at: now.clone(),
    };
    persist_analysis_and_recommendations(project_id, &analysis, &output, &now, &state)
}

/// Get the latest governance analysis for a project.
#[tauri::command]
pub fn governance_analysis_get(
    project_id: i64,
    state: State<'_, AppState>,
) -> Result<Option<GovernanceAnalysis>, OrqaError> {
    let conn = acquire_db(&state)?;
    governance_repo::get_latest_analysis(&conn, project_id)
}

/// List all recommendations for a project.
#[tauri::command]
pub fn recommendations_list(
    project_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Recommendation>, OrqaError> {
    let conn = acquire_db(&state)?;
    governance_repo::list_recommendations(&conn, project_id)
}

/// Update the status of a recommendation (approve, reject, etc.).
#[tauri::command]
pub fn recommendation_update(
    id: i64,
    status: String,
    state: State<'_, AppState>,
) -> Result<Recommendation, OrqaError> {
    let parsed = RecommendationStatus::parse(&status).ok_or_else(|| {
        OrqaError::Validation(format!(
            "invalid status '{status}': must be pending, approved, rejected, or applied"
        ))
    })?;
    let conn = acquire_db(&state)?;
    governance_repo::update_recommendation_status(&conn, id, &parsed)
}

/// Write an approved recommendation to disk and mark it applied.
#[tauri::command]
pub fn recommendation_apply(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Recommendation, OrqaError> {
    let (rec, project_path) = {
        let conn = acquire_db(&state)?;
        let rec = governance_repo::get_recommendation(&conn, id)?;
        let project = project_repo::get(&conn, rec.project_id)?;
        (rec, project.path)
    };

    write_recommendation_file(&rec, &project_path)?;

    let conn = acquire_db(&state)?;
    governance_repo::mark_recommendation_applied(&conn, id)
}

/// Apply all approved recommendations for a project.
#[tauri::command]
pub fn recommendations_apply_all(
    project_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Recommendation>, OrqaError> {
    let (recs, project_path) = {
        let conn = acquire_db(&state)?;
        let all = governance_repo::list_recommendations(&conn, project_id)?;
        let approved: Vec<_> = all
            .into_iter()
            .filter(|r| r.status == RecommendationStatus::Approved)
            .collect();
        let project = project_repo::get(&conn, project_id)?;
        (approved, project.path)
    };

    // Write all files before acquiring the DB lock so file I/O does not
    // hold the mutex. Any write failure aborts before any DB updates.
    for rec in &recs {
        write_recommendation_file(rec, &project_path)?;
    }

    // Acquire the lock once for all DB updates.
    let conn = acquire_db(&state)?;
    let mut applied = Vec::new();
    for rec in &recs {
        let updated = governance_repo::mark_recommendation_applied(&conn, rec.id)?;
        applied.push(updated);
    }
    Ok(applied)
}

// ── Internal helpers ──

/// Acquire the database lock and return it.
fn acquire_db<'a>(
    state: &'a State<'a, AppState>,
) -> Result<std::sync::MutexGuard<'a, rusqlite::Connection>, OrqaError> {
    state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))
}

/// Look up a project's filesystem path from the database.
fn get_project_path(project_id: i64, state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = acquire_db(state)?;
    let project = project_repo::get(&conn, project_id)?;
    Ok(project.path)
}

/// Create a governance session in the database and return its ID.
fn create_governance_session(
    project_id: i64,
    state: &State<'_, AppState>,
) -> Result<i64, OrqaError> {
    let conn = acquire_db(state)?;
    let session = session_repo::create(&conn, project_id, "auto", Some(GOVERNANCE_SYSTEM_PROMPT))?;
    Ok(session.id)
}

/// Persist the analysis and its recommendations, then return the saved analysis.
fn persist_analysis_and_recommendations(
    project_id: i64,
    analysis: &GovernanceAnalysis,
    output: &crate::domain::governance::ClaudeAnalysisOutput,
    now: &str,
    state: &State<'_, AppState>,
) -> Result<GovernanceAnalysis, OrqaError> {
    let conn = acquire_db(state)?;
    governance_repo::save_analysis(&conn, analysis)?;
    let analysis_id = conn.last_insert_rowid();
    let recs = build_recommendations(project_id, analysis_id, output, now);
    governance_repo::save_recommendations(&conn, &recs)?;
    governance_repo::get_latest_analysis(&conn, project_id)?
        .ok_or_else(|| OrqaError::Database("analysis not found after save".to_string()))
}

#[cfg(test)]
mod tests {
    use crate::db::init_memory_db;
    use crate::domain::governance::{
        GovernanceAnalysis, GovernanceScanResult, RecommendationStatus,
    };
    use crate::domain::governance_scanner::scan_governance;
    use crate::repo::{governance_repo, project_repo};

    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/tmp/test-project", None).expect("create project");
        conn
    }

    #[test]
    fn get_latest_analysis_returns_none_when_no_analysis() {
        let conn = setup();
        let result = governance_repo::get_latest_analysis(&conn, 1).expect("should succeed");
        assert!(result.is_none());
    }

    #[test]
    fn save_and_get_analysis_round_trip() {
        let conn = setup();
        let analysis = GovernanceAnalysis {
            id: 0,
            project_id: 1,
            scan_data: GovernanceScanResult {
                areas: vec![],
                coverage_ratio: 0.5,
            },
            summary: "Good governance".to_string(),
            strengths: vec!["Has rules".to_string()],
            gaps: vec!["No hooks".to_string()],
            session_id: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
        };

        governance_repo::save_analysis(&conn, &analysis).expect("save");
        let loaded = governance_repo::get_latest_analysis(&conn, 1)
            .expect("get")
            .expect("should exist");

        assert_eq!(loaded.project_id, 1);
        assert_eq!(loaded.summary, "Good governance");
        assert_eq!(loaded.strengths, vec!["Has rules".to_string()]);
        assert_eq!(loaded.gaps, vec!["No hooks".to_string()]);
    }

    #[test]
    fn recommendation_status_parse_valid() {
        assert_eq!(
            RecommendationStatus::parse("pending"),
            Some(RecommendationStatus::Pending)
        );
        assert_eq!(
            RecommendationStatus::parse("approved"),
            Some(RecommendationStatus::Approved)
        );
        assert_eq!(
            RecommendationStatus::parse("rejected"),
            Some(RecommendationStatus::Rejected)
        );
        assert_eq!(
            RecommendationStatus::parse("applied"),
            Some(RecommendationStatus::Applied)
        );
    }

    #[test]
    fn recommendation_status_parse_invalid() {
        assert!(RecommendationStatus::parse("invalid").is_none());
        assert!(RecommendationStatus::parse("").is_none());
        assert!(RecommendationStatus::parse("PENDING").is_none());
    }

    #[test]
    fn scan_governance_on_empty_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let result = scan_governance(dir.path()).expect("should succeed");
        assert_eq!(result.coverage_ratio, 0.0);
    }

    #[test]
    fn scan_governance_on_nonexistent_dir() {
        let result = scan_governance(std::path::Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn scan_governance_detects_rules_area() {
        let dir = tempfile::tempdir().expect("tempdir");
        let rules_dir = dir.path().join(".orqa").join("governance").join("rules");
        std::fs::create_dir_all(&rules_dir).expect("create rules dir");
        std::fs::write(rules_dir.join("RULE-001.md"), "# Test Rule\n").expect("write rule");

        let result = scan_governance(dir.path()).expect("should succeed");
        assert!(result.coverage_ratio > 0.0, "should detect rules area");
        let rules_area = result.areas.iter().find(|a| a.name == "rules");
        assert!(rules_area.is_some(), "should find rules area");
        assert!(
            rules_area.expect("rules").covered,
            "rules area should be covered"
        );
    }
}
