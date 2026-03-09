---
id: EPIC-002
title: "Settings UI for Thinking & Custom Prompt"
status: draft
priority: P1
milestone: MS-001
description: >
  Add UI controls for show_thinking toggle and custom system prompt
  fields that already exist in the backend.
created: 2026-03-07
updated: 2026-03-07
research-refs: []
docs-required:
  - docs/wireframes/settings-onboarding.md
  - docs/architecture/project-configuration.md
docs-produced:
  - docs/wireframes/settings-onboarding.md (update with thinking toggle and prompt sections)
scoring:
  pillar: 4
  impact: 4
  dependency: 2
  effort: 1
  score: 26.0
tags: [settings, thinking, system-prompt]
---

## Why P1

Can't control reasoning behaviour without these toggles. The custom system prompt is how dogfooding context reaches the AI.

## Tasks

- [ ] Add `show_thinking` toggle in Settings > Model section
- [ ] Add `custom_system_prompt` textarea in Settings > Project section
- [ ] Add "View auto-generated prompt" collapsible preview (reuse `build_system_prompt()`)
