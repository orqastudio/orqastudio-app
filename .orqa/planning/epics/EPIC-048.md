---
id: EPIC-048
title: Artifact Integrity — Body Templates and Cross-Linking
description: "Establish consistent body structure for all non-doc artifact types with pre-commit linting, implement robust inter-artifact linking with backend ID-to-path resolution, add markdown cross-linking in rendered bodies, and add a file watcher to keep the artifact index live."
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
  effort: 4
  risk: 3
  score: 17
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

**Enforcement approach:** Two levels:

1. **Documentation** — templates documented in artifact-framework.md
2. **Linting** — pre-commit hook checks for required `## Heading` patterns, driven by template definitions in each type's schema.json

Body templates are defined in schema.json alongside frontmatter schemas — one source of truth for all structural expectations per artifact type.

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
- `docs-required`/`docs-produced` render as path links, validated against disk during scan
- All artifact ID fields render as `ArtifactLink` with resolution
- **Broken links** styled with broken-link icon + app warning colour token (visually distinct, not hidden)
- Broken `docs-required`/`docs-produced` paths flagged in the UI

### Part 4: Markdown Cross-Linking

- Regex pass over rendered markdown in `MarkdownRenderer` matching all known artifact ID patterns (EPIC-NNN, TASK-NNN, AD-NNN, MS-NNN, IDEA-NNN, IMPL-NNN, RES-NNN, PILLAR-NNN, RULE-NNN)
- Wrap matches in clickable elements that call `navigateToArtifact`
- Always-on — no configuration needed

### Part 5: File Watcher for Artifact Index

- Watch `.orqa/` for file system changes (create, modify, delete, rename)
- Rebuild artifact index on change (debounced)
- Emit NavTree refresh event to frontend so the sidebar stays current
- Replaces manual rescan — the index is always up to date

## Out of Scope

- Artifact graph visualization (node graph rendering) — separate epic
- App-assisted template pre-population (artifact editor) — deferred to EPIC-004

## Tasks

| Task | Title | Scope |
|------|-------|-------|
| TASK-070 | Document body templates in artifact-framework.md and schema.json | .orqa/documentation/, .orqa/**/schema.json |
| TASK-071 | Add body template linting to pre-commit hook | .githooks/validate-schema.mjs |
| TASK-072 | Backfill existing artifacts to match body templates | .orqa/planning/, .orqa/governance/ |
| TASK-073 | Add artifact ID to DocNode and build ArtifactIndex | src-tauri/, ui/lib/types/ |
| TASK-074 | Add resolve_artifact_id Tauri command | src-tauri/src/commands/ |
| TASK-075 | Replace prefix-map navigation with path-based resolution | ui/lib/stores/, ui/lib/components/ |
| TASK-076 | Broken link styling and path validation | ui/lib/components/artifact/ |
| TASK-077 | Markdown cross-linking in MarkdownRenderer | ui/lib/components/shared/MarkdownRenderer.svelte |
| TASK-078 | File watcher for .orqa/ with artifact index rebuild | src-tauri/src/ |

## Dependency Chain

```
TASK-070 (templates + schema) ──> TASK-071 (linting) ──> TASK-072 (backfill)

TASK-073 (DocNode id + ArtifactIndex) ──> TASK-074 (resolve command) ──> TASK-075 (frontend nav) ──> TASK-076 (broken links)
                                                                                                  ──> TASK-077 (markdown cross-links)

TASK-073 (ArtifactIndex) ──> TASK-078 (file watcher refreshes index)
```

The two tracks (body templates and linking) are independent and can be parallelized. Within the linking track, markdown cross-linking and broken link styling can also be parallelized after TASK-075 is complete.
