---
id: TASK-162
title: Remove brackets from artifact links in all .orqa/ artifacts
description: Strip square brackets from artifact link display text across all .orqa/ markdown files — change [EPIC-001](EPIC-001) to EPIC-001 plain text or bare link format.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Identify the current artifact link format used across .orqa/ files
  - Replace bracketed link format with a cleaner alternative across all artifacts
  - Ensure the app's markdown renderer still resolves artifact links correctly after the change
acceptance:
  - No artifact links use the [ID](ID) bracketed display format
  - Artifact links still resolve correctly in the app's markdown renderer
  - All .orqa/ markdown files updated consistently
  - Research files (status surpassed) are exempt per RULE-014
---
## What

Artifact links currently use the markdown format `[EPIC-001](EPIC-001)` which renders with brackets as visual noise in the app's display. Since the app controls how artifacts are rendered, the bracketed display text is unnecessary — the bare ID or a cleaner link format would be visually cleaner.

## How

1. Determine the target format — either bare `EPIC-001` with auto-linking by the renderer, or a bracket-free markdown link format
2. Verify the app's markdown renderer can handle the new format (auto-link artifact IDs, or support a simpler syntax)
3. Find-and-replace across all non-surpassed `.orqa/` markdown files
4. Skip research files with `status: surpassed` per RULE-014 (historical records are immutable)
5. Update RULE-044 (artifact link format rule, if created by TASK-144) to reflect the new convention

## Verification

- [ ] No `[ID](ID)` format artifact links remain in active .orqa/ files
- [ ] Artifact links still render and navigate correctly in the app
- [ ] Surpassed research files are unchanged
- [ ] Consistent format across all artifact types
