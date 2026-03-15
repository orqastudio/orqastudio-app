---
id: TASK-436
title: Field display improvements — badges, checkbox icons, display order
description: Improve frontmatter field display with badges for maturity/category/version, checkbox icons for booleans, and better field ordering.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-067
depends-on: []
assignee: null
skills: []
acceptance:
  - Maturity renders as badge above recurrence
  - Category and version as badges
  - Boolean fields show checkbox icon
  - All via FrontmatterHeader config
relationships:
  - target: EPIC-067
    type: delivers
    rationale: Better field display improves artifact readability
  - target: EPIC-067
    type: belongs-to
    rationale: Task belongs to this epic
---

## Scope

Update FrontmatterHeader.svelte CHIP_FIELDS, BOOLEAN_FIELDS, and field ordering configuration. Add badge rendering for maturity, category, and version fields. Add checkbox icon rendering for boolean fields.
