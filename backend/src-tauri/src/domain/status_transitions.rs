use serde::{Deserialize, Serialize};

use crate::domain::artifact_graph::ArtifactGraph;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

/// A status transition proposed by the evaluation engine.
///
/// Transitions are never applied directly — they are returned to the caller
/// so that the frontend can present them to the user before any mutation
/// occurs. `auto_apply` signals that the transition is unambiguous and may be
/// applied programmatically without human confirmation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedTransition {
    /// Artifact identifier, e.g. `"EPIC-048"`.
    pub artifact_id: String,
    /// Relative path from the project root, e.g. `".orqa/delivery/epics/EPIC-048.md"`.
    pub artifact_path: String,
    /// Current `status` frontmatter value.
    pub current_status: String,
    /// Status value to transition to.
    pub proposed_status: String,
    /// Human-readable explanation of why this transition is proposed.
    pub reason: String,
    /// When `true` the transition is unambiguous and can be applied without
    /// explicit human approval (e.g. a task becoming blocked because a
    /// dependency is not yet complete).
    pub auto_apply: bool,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Evaluate the artifact graph and return all transitions that should occur.
///
/// The function is pure — it only reads `graph` and never modifies it.
/// All five rules are evaluated in a single pass:
///
/// 1. All tasks completed → epic to `review`
/// 2. All P1 epics completed → milestone to `review`
/// 3. Dependency not met → task to `blocked`
/// 4. All dependencies met → task from `blocked` to `ready`
/// 5. Lesson recurrence ≥ 2 → lesson to `review`
pub fn evaluate_transitions(graph: &ArtifactGraph) -> Vec<ProposedTransition> {
    let mut proposals: Vec<ProposedTransition> = Vec::new();

    propose_epic_to_review(graph, &mut proposals);
    propose_milestone_to_review(graph, &mut proposals);
    propose_task_blocked(graph, &mut proposals);
    propose_task_unblocked(graph, &mut proposals);
    propose_lesson_to_review(graph, &mut proposals);

    proposals
}

// ---------------------------------------------------------------------------
// Rule 1 — all tasks completed → epic to review
// ---------------------------------------------------------------------------

fn propose_epic_to_review(graph: &ArtifactGraph, proposals: &mut Vec<ProposedTransition>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "epic" {
            continue;
        }
        let Some(current_status) = node.status.as_deref() else {
            continue;
        };
        if current_status != "active" && current_status != "in-progress" {
            continue;
        }

        // Collect all tasks that reference this epic.
        let related_tasks: Vec<&crate::domain::artifact_graph::ArtifactNode> = graph
            .nodes
            .values()
            .filter(|n| {
                if n.artifact_type != "task" {
                    return false;
                }
                // Check `epic` frontmatter field reference.
                let epic_field = n
                    .frontmatter
                    .get("epic")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if epic_field == node.id {
                    return true;
                }
                // Check `delivers` relationship edge.
                n.references_out.iter().any(|r| {
                    r.relationship_type.as_deref() == Some("delivers") && r.target_id == node.id
                })
            })
            .collect();

        if related_tasks.is_empty() {
            continue;
        }

        let all_completed = related_tasks.iter().all(|t| {
            t.status.as_deref() == Some("completed") || t.status.as_deref() == Some("done")
        });

        if all_completed {
            proposals.push(ProposedTransition {
                artifact_id: node.id.clone(),
                artifact_path: node.path.clone(),
                current_status: current_status.to_owned(),
                proposed_status: "review".to_owned(),
                reason: format!("All {} related task(s) are completed", related_tasks.len()),
                auto_apply: false,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Rule 2 — all P1 epics completed → milestone to review
// ---------------------------------------------------------------------------

/// Return `true` if `node` is a P1 epic that belongs to `milestone_id`.
fn is_p1_epic_for_milestone(
    node: &crate::domain::artifact_graph::ArtifactNode,
    milestone_id: &str,
) -> bool {
    if node.artifact_type != "epic" {
        return false;
    }
    let milestone_ref = node
        .frontmatter
        .get("milestone")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if milestone_ref != milestone_id {
        return false;
    }
    node.frontmatter
        .get("priority")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        == "P1"
}

fn propose_milestone_to_review(graph: &ArtifactGraph, proposals: &mut Vec<ProposedTransition>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "milestone" {
            continue;
        }
        let Some(current_status) = node.status.as_deref() else {
            continue;
        };
        if current_status != "active" {
            continue;
        }

        let p1_epics: Vec<&crate::domain::artifact_graph::ArtifactNode> = graph
            .nodes
            .values()
            .filter(|n| is_p1_epic_for_milestone(n, &node.id))
            .collect();

        if p1_epics.is_empty() {
            continue;
        }

        let all_completed = p1_epics.iter().all(|e| {
            e.status.as_deref() == Some("completed") || e.status.as_deref() == Some("done")
        });

        if all_completed {
            proposals.push(ProposedTransition {
                artifact_id: node.id.clone(),
                artifact_path: node.path.clone(),
                current_status: current_status.to_owned(),
                proposed_status: "review".to_owned(),
                reason: format!(
                    "All {} P1 epic(s) for this milestone are completed",
                    p1_epics.len()
                ),
                auto_apply: false,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Rule 3 — dependency not met → task to blocked
// ---------------------------------------------------------------------------

fn propose_task_blocked(graph: &ArtifactGraph, proposals: &mut Vec<ProposedTransition>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "task" {
            continue;
        }
        let Some(current_status) = node.status.as_deref() else {
            continue;
        };
        if current_status != "ready" && current_status != "todo" {
            continue;
        }

        let depends_on = collect_depends_on(&node.frontmatter);
        if depends_on.is_empty() {
            continue;
        }

        let blocking: Vec<&str> = depends_on
            .iter()
            .filter(|dep_id| {
                match graph.nodes.get(dep_id.as_str()) {
                    Some(dep) => {
                        dep.status.as_deref() != Some("completed")
                            && dep.status.as_deref() != Some("done")
                    }
                    // Unknown dependency — treat as blocking to be safe.
                    None => true,
                }
            })
            .map(String::as_str)
            .collect();

        if !blocking.is_empty() {
            proposals.push(ProposedTransition {
                artifact_id: node.id.clone(),
                artifact_path: node.path.clone(),
                current_status: current_status.to_owned(),
                proposed_status: "blocked".to_owned(),
                reason: format!("Dependency {} not completed", blocking.join(", ")),
                auto_apply: true,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Rule 4 — all dependencies met → task from blocked to ready
// ---------------------------------------------------------------------------

fn propose_task_unblocked(graph: &ArtifactGraph, proposals: &mut Vec<ProposedTransition>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "task" {
            continue;
        }
        let Some(current_status) = node.status.as_deref() else {
            continue;
        };
        if current_status != "blocked" {
            continue;
        }

        let depends_on = collect_depends_on(&node.frontmatter);
        if depends_on.is_empty() {
            // Blocked with no dependencies — don't auto-unblock; something
            // else caused the block.
            continue;
        }

        let all_complete = depends_on.iter().all(|dep_id| {
            graph
                .nodes
                .get(dep_id.as_str())
                .and_then(|n| n.status.as_deref())
                .is_some_and(|s| s == "completed" || s == "done")
        });

        if all_complete {
            proposals.push(ProposedTransition {
                artifact_id: node.id.clone(),
                artifact_path: node.path.clone(),
                current_status: current_status.to_owned(),
                proposed_status: "ready".to_owned(),
                reason: format!(
                    "All {} dependency/dependencies are now completed",
                    depends_on.len()
                ),
                auto_apply: true,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Rule 5 — lesson recurrence ≥ 2 → lesson to review
// ---------------------------------------------------------------------------

fn propose_lesson_to_review(graph: &ArtifactGraph, proposals: &mut Vec<ProposedTransition>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "lesson" {
            continue;
        }
        let Some(current_status) = node.status.as_deref() else {
            continue;
        };
        if current_status != "active" && current_status != "recurring" {
            continue;
        }

        let recurrence = node
            .frontmatter
            .get("recurrence")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);

        if recurrence >= 2 {
            proposals.push(ProposedTransition {
                artifact_id: node.id.clone(),
                artifact_path: node.path.clone(),
                current_status: current_status.to_owned(),
                proposed_status: "review".to_owned(),
                reason: format!(
                    "Lesson recurrence is {recurrence} (threshold: 2) — ready for promotion"
                ),
                auto_apply: false,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Extract the `depends-on` array from a node's frontmatter JSON value.
///
/// Handles both YAML sequences (serialized as JSON arrays) and a single
/// scalar string.
fn collect_depends_on(frontmatter: &serde_json::Value) -> Vec<String> {
    match frontmatter.get("depends-on") {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect(),
        Some(serde_json::Value::String(s)) if !s.trim().is_empty() => vec![s.clone()],
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::domain::artifact_graph::{ArtifactNode, ArtifactRef};

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_node(
        id: &str,
        artifact_type: &str,
        status: &str,
        frontmatter: serde_json::Value,
    ) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            path: format!(".orqa/{id}.md"),
            artifact_type: artifact_type.to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some(status.to_owned()),
            priority: None,
            frontmatter,
            references_out: Vec::new(),
            references_in: Vec::new(),
        }
    }

    fn make_graph(nodes: Vec<ArtifactNode>) -> ArtifactGraph {
        let mut map = HashMap::new();
        for n in nodes {
            map.insert(n.id.clone(), n);
        }
        ArtifactGraph {
            nodes: map,
            path_index: HashMap::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Rule 1 — epic to review
    // -----------------------------------------------------------------------

    #[test]
    fn rule1_proposes_epic_review_when_all_tasks_done() {
        let epic = make_node("EPIC-001", "epic", "in-progress", serde_json::json!({}));
        let task1 = make_node(
            "TASK-001",
            "task",
            "completed",
            serde_json::json!({ "epic": "EPIC-001" }),
        );
        let task2 = make_node(
            "TASK-002",
            "task",
            "done",
            serde_json::json!({ "epic": "EPIC-001" }),
        );
        let graph = make_graph(vec![epic, task1, task2]);

        let proposals = evaluate_transitions(&graph);
        let epic_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "EPIC-001")
            .collect();

        assert_eq!(epic_proposals.len(), 1);
        assert_eq!(epic_proposals[0].proposed_status, "review");
        assert!(!epic_proposals[0].auto_apply);
    }

    #[test]
    fn rule1_no_proposal_when_task_still_active() {
        let epic = make_node("EPIC-002", "epic", "in-progress", serde_json::json!({}));
        let task1 = make_node(
            "TASK-003",
            "task",
            "completed",
            serde_json::json!({ "epic": "EPIC-002" }),
        );
        let task2 = make_node(
            "TASK-004",
            "task",
            "in-progress",
            serde_json::json!({ "epic": "EPIC-002" }),
        );
        let graph = make_graph(vec![epic, task1, task2]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals
            .iter()
            .all(|p| p.artifact_id != "EPIC-002" || p.proposed_status != "review"));
    }

    #[test]
    fn rule1_no_proposal_when_no_related_tasks() {
        let epic = make_node("EPIC-003", "epic", "active", serde_json::json!({}));
        let graph = make_graph(vec![epic]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "EPIC-003"));
    }

    #[test]
    fn rule1_accepts_delivers_relationship() {
        let epic = make_node("EPIC-010", "epic", "active", serde_json::json!({}));
        let mut task = make_node("TASK-010", "task", "completed", serde_json::json!({}));
        task.references_out.push(ArtifactRef {
            target_id: "EPIC-010".to_owned(),
            field: "relationships".to_owned(),
            source_id: "TASK-010".to_owned(),
            relationship_type: Some("delivers".to_owned()),
        });
        let graph = make_graph(vec![epic, task]);

        let proposals = evaluate_transitions(&graph);
        let epic_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "EPIC-010")
            .collect();

        assert_eq!(epic_proposals.len(), 1);
        assert_eq!(epic_proposals[0].proposed_status, "review");
    }

    // -----------------------------------------------------------------------
    // Rule 2 — milestone to review
    // -----------------------------------------------------------------------

    #[test]
    fn rule2_proposes_milestone_review_when_all_p1_epics_done() {
        let ms = make_node("MS-001", "milestone", "active", serde_json::json!({}));
        let epic1 = make_node(
            "EPIC-011",
            "epic",
            "completed",
            serde_json::json!({ "milestone": "MS-001", "priority": "P1" }),
        );
        let epic2 = make_node(
            "EPIC-012",
            "epic",
            "completed",
            serde_json::json!({ "milestone": "MS-001", "priority": "P1" }),
        );
        let graph = make_graph(vec![ms, epic1, epic2]);

        let proposals = evaluate_transitions(&graph);
        let ms_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "MS-001")
            .collect();

        assert_eq!(ms_proposals.len(), 1);
        assert_eq!(ms_proposals[0].proposed_status, "review");
        assert!(!ms_proposals[0].auto_apply);
    }

    #[test]
    fn rule2_no_proposal_when_p1_epic_not_done() {
        let ms = make_node("MS-002", "milestone", "active", serde_json::json!({}));
        let epic1 = make_node(
            "EPIC-013",
            "epic",
            "completed",
            serde_json::json!({ "milestone": "MS-002", "priority": "P1" }),
        );
        let epic2 = make_node(
            "EPIC-014",
            "epic",
            "in-progress",
            serde_json::json!({ "milestone": "MS-002", "priority": "P1" }),
        );
        let graph = make_graph(vec![ms, epic1, epic2]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "MS-002"));
    }

    #[test]
    fn rule2_ignores_p2_epics_when_checking_completion() {
        let ms = make_node("MS-003", "milestone", "active", serde_json::json!({}));
        let epic_p1 = make_node(
            "EPIC-015",
            "epic",
            "completed",
            serde_json::json!({ "milestone": "MS-003", "priority": "P1" }),
        );
        // P2 epic is still in-progress — should not block milestone review.
        let epic_p2 = make_node(
            "EPIC-016",
            "epic",
            "in-progress",
            serde_json::json!({ "milestone": "MS-003", "priority": "P2" }),
        );
        let graph = make_graph(vec![ms, epic_p1, epic_p2]);

        let proposals = evaluate_transitions(&graph);
        let ms_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "MS-003")
            .collect();

        assert_eq!(ms_proposals.len(), 1);
        assert_eq!(ms_proposals[0].proposed_status, "review");
    }

    #[test]
    fn rule2_no_proposal_when_no_p1_epics() {
        let ms = make_node("MS-004", "milestone", "active", serde_json::json!({}));
        let epic_p2 = make_node(
            "EPIC-017",
            "epic",
            "completed",
            serde_json::json!({ "milestone": "MS-004", "priority": "P2" }),
        );
        let graph = make_graph(vec![ms, epic_p2]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "MS-004"));
    }

    // -----------------------------------------------------------------------
    // Rule 3 — task to blocked
    // -----------------------------------------------------------------------

    #[test]
    fn rule3_proposes_blocked_when_dependency_not_done() {
        let dep = make_node("TASK-020", "task", "in-progress", serde_json::json!({}));
        let task = make_node(
            "TASK-021",
            "task",
            "ready",
            serde_json::json!({ "depends-on": ["TASK-020"] }),
        );
        let graph = make_graph(vec![dep, task]);

        let proposals = evaluate_transitions(&graph);
        let blocked: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "TASK-021")
            .collect();

        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].proposed_status, "blocked");
        assert!(blocked[0].auto_apply);
    }

    #[test]
    fn rule3_no_proposal_when_all_dependencies_done() {
        let dep = make_node("TASK-022", "task", "completed", serde_json::json!({}));
        let task = make_node(
            "TASK-023",
            "task",
            "ready",
            serde_json::json!({ "depends-on": ["TASK-022"] }),
        );
        let graph = make_graph(vec![dep, task]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-023"));
    }

    #[test]
    fn rule3_no_proposal_when_no_dependencies() {
        let task = make_node("TASK-024", "task", "ready", serde_json::json!({}));
        let graph = make_graph(vec![task]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-024"));
    }

    #[test]
    fn rule3_handles_scalar_depends_on() {
        let dep = make_node("TASK-025", "task", "in-progress", serde_json::json!({}));
        let task = make_node(
            "TASK-026",
            "task",
            "todo",
            serde_json::json!({ "depends-on": "TASK-025" }),
        );
        let graph = make_graph(vec![dep, task]);

        let proposals = evaluate_transitions(&graph);
        let blocked: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "TASK-026")
            .collect();

        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].proposed_status, "blocked");
    }

    // -----------------------------------------------------------------------
    // Rule 4 — task unblocked
    // -----------------------------------------------------------------------

    #[test]
    fn rule4_proposes_ready_when_all_deps_complete() {
        let dep = make_node("TASK-030", "task", "completed", serde_json::json!({}));
        let task = make_node(
            "TASK-031",
            "task",
            "blocked",
            serde_json::json!({ "depends-on": ["TASK-030"] }),
        );
        let graph = make_graph(vec![dep, task]);

        let proposals = evaluate_transitions(&graph);
        let unblocked: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "TASK-031")
            .collect();

        assert_eq!(unblocked.len(), 1);
        assert_eq!(unblocked[0].proposed_status, "ready");
        assert!(unblocked[0].auto_apply);
    }

    #[test]
    fn rule4_no_proposal_when_dep_still_incomplete() {
        let dep = make_node("TASK-032", "task", "in-progress", serde_json::json!({}));
        let task = make_node(
            "TASK-033",
            "task",
            "blocked",
            serde_json::json!({ "depends-on": ["TASK-032"] }),
        );
        let graph = make_graph(vec![dep, task]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-033"));
    }

    #[test]
    fn rule4_no_proposal_for_blocked_task_with_no_deps() {
        let task = make_node("TASK-034", "task", "blocked", serde_json::json!({}));
        let graph = make_graph(vec![task]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-034"));
    }

    // -----------------------------------------------------------------------
    // Rule 5 — lesson to review
    // -----------------------------------------------------------------------

    #[test]
    fn rule5_proposes_review_when_recurrence_at_threshold() {
        let lesson = make_node(
            "IMPL-001",
            "lesson",
            "active",
            serde_json::json!({ "recurrence": 2 }),
        );
        let graph = make_graph(vec![lesson]);

        let proposals = evaluate_transitions(&graph);
        let lesson_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "IMPL-001")
            .collect();

        assert_eq!(lesson_proposals.len(), 1);
        assert_eq!(lesson_proposals[0].proposed_status, "review");
        assert!(!lesson_proposals[0].auto_apply);
    }

    #[test]
    fn rule5_proposes_review_when_recurrence_exceeds_threshold() {
        let lesson = make_node(
            "IMPL-002",
            "lesson",
            "recurring",
            serde_json::json!({ "recurrence": 5 }),
        );
        let graph = make_graph(vec![lesson]);

        let proposals = evaluate_transitions(&graph);
        let lesson_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "IMPL-002")
            .collect();

        assert_eq!(lesson_proposals.len(), 1);
        assert_eq!(lesson_proposals[0].proposed_status, "review");
    }

    #[test]
    fn rule5_no_proposal_when_recurrence_below_threshold() {
        let lesson = make_node(
            "IMPL-003",
            "lesson",
            "active",
            serde_json::json!({ "recurrence": 1 }),
        );
        let graph = make_graph(vec![lesson]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "IMPL-003"));
    }

    #[test]
    fn rule5_no_proposal_when_lesson_already_promoted() {
        let lesson = make_node(
            "IMPL-004",
            "lesson",
            "promoted",
            serde_json::json!({ "recurrence": 3 }),
        );
        let graph = make_graph(vec![lesson]);

        let proposals = evaluate_transitions(&graph);
        assert!(proposals.iter().all(|p| p.artifact_id != "IMPL-004"));
    }

    // -----------------------------------------------------------------------
    // Empty graph
    // -----------------------------------------------------------------------

    #[test]
    fn empty_graph_returns_no_proposals() {
        let graph = make_graph(vec![]);
        let proposals = evaluate_transitions(&graph);
        assert!(proposals.is_empty());
    }
}
