---
id: TASK-432
title: "Relationships panel — equal columns, overflow toggle, status dots"
description: "Redesign the relationships panel with equal-width columns, expandable overflow, and status dots from graph node data."
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
  - "Equal width label/value columns"
  - "One row per relationship type with '...' to expand overflow, 'hide' to collapse"
  - "Each chip shows status dot from graph node data"
relationships:
  - target: EPIC-067
    type: delivers
    rationale: "Improved relationships panel provides better artifact context at a glance"
---

## Scope

Redesign ReferencesPanel.svelte and RelationshipsList.svelte. Implement equal-width columns for label and value. Add overflow toggle per relationship type row. Show status dots on relationship chips using enriched graph node data from TASK-429.
