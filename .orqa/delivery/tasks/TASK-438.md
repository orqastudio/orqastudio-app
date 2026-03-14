---
id: TASK-438
title: "Actions needed icon in artifact list + epics without tasks"
description: "Show action-needed indicators in artifact list items and flag epics that have no tasks referencing them."
status: todo
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-067
depends-on:
  - TASK-429
assignee: null
skills: []
acceptance:
  - "Artifacts with pending actions show icon indicator in ArtifactListItem"
  - "Epics with no tasks referencing them show as action needed"
relationships:
  - target: EPIC-067
    type: delivers
    rationale: "Action indicators in list view surface what needs attention without opening each artifact"
---

## Scope

Update ActionsNeeded logic to detect epics without tasks. Update ArtifactListItem.svelte to show an action-needed icon indicator when the artifact has pending actions.
