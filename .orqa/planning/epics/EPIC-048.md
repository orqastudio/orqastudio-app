---
id: EPIC-048
title: Artifact Integrity — Body Templates and Cross-Linking
description: "Establish consistent body structure for all non-doc artifact types and implement robust inter-artifact linking with backend ID-to-path resolution, replacing the fragile frontend prefix-map approach."
status: draft
priority: P1
created: 2026-03-10
updated: 2026-03-10
milestone: MS-001
pillars:
  - PILLAR-001
  - PILLAR-002
research-refs:
  - RES-032
  - RES-033
docs-required:
  - .orqa/planning/research/RES-032.md
  - .orqa/planning/research/RES-033.md
  - .orqa/documentation/product/artifact-framework.md
docs-produced:
  - .orqa/documentation/product/artifact-framework.md
scoring:
  user-value: 5
  pillar-alignment: 5
  dependency-weight: 4
  effort: 3
  risk: 2
  score: 19
---
## Context

Two systemic gaps identified during dogfooding prevent the artifact system from being self-consistent:

1. **Body structure is freeform** — Artifact frontmatter is now schema-enforced (JSON Schema + pre-commit validation), but everything below the `---` is whatever the author invents. Some types have naturally converged (pillars, milestones, decisions, lessons) while others vary widely (epics, tasks) or are nearly empty (ideas). This makes it harder to find information and prevents the app from eventually parsing or rendering structured sections.

2. **Cross-linking is fragile** — ArtifactLink navigation uses a hardcoded `ARTIFACT_PREFIX_MAP` in the frontend and `label.startsWith(pendingId)` string matching. This breaks for tree-structured directories (documentation), misses artifact types without prefix entries (RES, PILLAR, RULE), and silently fails when frontmatter titles don't match filenames. There is no backend ID-to-path resolution.

Both gaps directly impact Pillar 1 (Clarity Through Structure) — the artifact system should make information findable and navigable. The linking gap also impacts Pillar 2 (Learning Through Reflection) — lessons, decisions, and research docs reference each other, and broken links prevent traceability.

## Implementation Design

### Part 1: Body Templates (RES-032)

Document and enforce minimum body structure for each artifact type:

| Type | Required Sections | Status |
|------|-------------------|--------|
| Pillar | What This Pillar Means, Examples, Anti-Patterns, Conflict Resolution | Already consistent |
| Milestone | Context, Epics, Completion Criteria | Already consistent |
| Decision | Decision, Rationale, Consequences | Already consistent |
| Lesson | Pattern, Fix | Already consistent |
| Epic | Context, Implementation Design, Tasks, Out of Scope (optional) | Needs enforcement |
| Task | What, How, Verification | Needs enforcement |
| Idea | Motivation, Sketch (optional) | Needs enforcement |
| Rule | Opening paragraph, domain sections, FORBIDDEN, Related Rules | Semi-structured |
| Research | Intentionally freeform | No template |

**Enforcement approach:** Documentation-first (document templates in artifact-framework.md). Future: pre-commit linting for required `## Heading` patterns.

### Part 2: Artifact Index and Resolution (RES-033)

**Backend:**
- Extract `id` field during `artifact_scan_tree` scan
- Build `ArtifactIndex` (HashMap<String, String>) mapping IDs to file paths
- Store in `AppState`
- Add `resolve_artifact_id` Tauri command (HashMap lookup)
- Add `id` field to `DocNode` struct (Rust + TypeScript)

**Frontend:**
- Add `navigateToPath(path)` to NavigationStore — walks full NavTree including tree children
- Replace `ARTIFACT_PREFIX_MAP` + `label.startsWith()` with path-based navigation
- Remove `if (isTree) return;` guard in ArtifactNav
- `ArtifactLink` uses backend resolution or local NavTree ID lookup
- Unknown IDs show "not found" indicator instead of silently failing

### Part 3: Link Rendering Polish

- `FrontmatterHeader` distinguishes ID links (resolve via index) from path links (navigate directly)
- `docs-required`/`docs-produced` render as path links
- All artifact ID fields render as `ArtifactLink` with resolution
- Add missing prefix entries for RES, PILLAR, RULE (temporary until full resolution replaces the map)

## Out of Scope

- Markdown body cross-linking (auto-wrapping artifact IDs in rendered markdown) — separate future work
- Artifact graph visualization (node graph rendering) — separate epic
- Body template linting in pre-commit hook — future enforcement level, not this epic

## Tasks

| Task | Title | Scope |
|------|-------|-------|
| TASK-070 | Document body templates in artifact-framework.md | .orqa/documentation/ |
| TASK-071 | Backfill existing artifacts to match body templates | .orqa/planning/, .orqa/governance/ |
| TASK-072 | Add artifact ID to DocNode and build ArtifactIndex | src-tauri/, ui/lib/types/ |
| TASK-073 | Add resolve_artifact_id Tauri command | src-tauri/src/commands/ |
| TASK-074 | Replace prefix-map navigation with path-based resolution | ui/lib/stores/, ui/lib/components/ |
| TASK-075 | Polish link rendering in FrontmatterHeader | ui/lib/components/artifact/ |

## Dependency Chain

```
TASK-070 (document templates) ─────────────────────────────────> TASK-071 (backfill artifacts)
TASK-072 (DocNode id + ArtifactIndex) ──> TASK-073 (resolve command) ──> TASK-074 (frontend nav) ──> TASK-075 (link polish)
```

The two tracks (body templates and linking) are independent and can be parallelized.
