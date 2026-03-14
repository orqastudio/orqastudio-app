---
id: EPIC-073
title: "UAT round 3 — dashboard redesign, artifact discipline, broken links"
description: "UAT findings from reviewing the output of EPIC-067/068/069/070/071/072 implementation. Dashboard needs holistic redesign not just card rearrangement. Agent artifact creation discipline is failing — 57 warnings from a single implementation round."
status: draft
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
deadline: null
milestone: MS-001
horizon: active
pillars:
  - PILLAR-001
  - PILLAR-003
depends-on: []
research-refs: []
docs-required: []
docs-produced: []
relationships:
  - target: EPIC-068
    type: informed-by
    rationale: "Dashboard grid layout was insufficient — needs holistic redesign"
  - target: EPIC-067
    type: informed-by
    rationale: "Implementation round created integrity errors despite rules existing"
---

## Context

UAT round 3 after implementing 31 tasks across 6 epics. Three findings so far — collection paused to reset context.

## UAT Findings

| # | Finding | Type | Area |
|---|---------|------|------|
| F1 | Dashboard is just the same cards rearranged in a grid — needs holistic redesign of what data is shown and how. Research dashboard layouts from similar products for inspiration | ux | dashboard |
| F2 | 57 warnings + 1 error created by this implementation round — agents are not following artifact creation rules. Integrity errors should surface edge cases and human error, not AI implementation failures. Knowledge injection / enforcement not working for artifact creation discipline | bug | process |
| F3 | Broken link: RES-035 references TASK-069 in body but it doesn't resolve | bug | data |

## Implementation Design

TBD — needs systemic analysis after full collection.

## Tasks

TBD — pending Phase 2 analysis.
