---
id: TASK-441
title: "Dashboard grid layout system"
description: "Replace the vertical card stack dashboard with a CSS grid layout that supports future drag-drop and plugin widgets."
status: done
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-068
depends-on: []
assignee: null
skills: []
acceptance:
  - "Dashboard uses a grid layout instead of vertical card stack"
  - "Architecture supports future drag-drop and plugin widgets"
  - "Widgets can span columns/rows"
relationships:
  - target: EPIC-068
    type: delivers
    rationale: "Grid layout is the foundation for a more useful dashboard"
---

## Scope

Redesign ProjectDashboard.svelte with CSS grid. Create a widget slot system that allows widgets to declare their grid span. Maintain existing widget functionality in the new layout.
