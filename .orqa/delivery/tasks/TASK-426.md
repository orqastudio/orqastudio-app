---
id: TASK-426
title: Initial publish of tier-0 packages to GitHub Packages
description: Create GitHub releases for types, eslint-config, and test-config to trigger the publish workflows. These have no orqa dependencies so they can be published first.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-066
depends-on:
  - TASK-425
assignee: null
skills:
  - SKILL-034
acceptance:
  - "@orqastudio/types v0.1.0 published to GitHub Packages"
  - "@orqastudio/eslint-config v0.1.0 published to GitHub Packages"
  - "@orqastudio/test-config v0.1.0 published to GitHub Packages"
  - All three installable via npm install @orqastudio/types (with .npmrc configured)
relationships:
  - target: EPIC-066
    type: delivers
    rationale: First packages published — unblocks tier-1 publishing
  - target: TASK-425
    type: informed-by
    rationale: CI must pass before releases can be created
  - target: EPIC-066
    type: belongs-to
    rationale: Task belongs to this epic
---

## Scope

For each tier-0 package:
1. Verify CI passes on GitHub Actions
2. Create a GitHub release tagged `v0.1.0`
3. Verify the publish workflow runs and succeeds
4. Test installation: `npm install @orqastudio/types` with `.npmrc` configured
