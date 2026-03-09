---
id: IDEA-027
title: "Native Hooks System"
status: captured
pillar:
  - clarity-through-structure
description: >
  Design a hooks layer that works in both app and CLI contexts, with
  richer lifecycle events and backwards compatibility with Claude Code hooks.
research-needed:
  - Audit current Claude hooks in .claude/settings.json and what they enforce
  - Research Claude hookify plugin as inspiration for CLI compatibility
  - Design a hooks layer that works in both app and CLI contexts
  - Define hook lifecycle (pre-commit, session-start, pre-delegation, post-delegation, etc.)
  - Determine which hooks should be platform-managed vs user-defined
  - Consider how hooks relate to the three-layer governance model
promoted-to: null
tags: [hooks, governance, enforcement, cli-compatibility, platform]
---

## Problem

OrqaStudio currently relies on Claude Code hooks (`.claude/settings.json`) for process enforcement — session-start checks, pre-commit reminders. But these only work in the CLI context. The app needs its own hooks layer that:

1. Enforces the same rules when running through the app (not just CLI)
2. Provides backwards compatibility with Claude hooks for CLI users
3. Supports richer hook types (pre-delegation, post-delegation, artifact-change, etc.)
4. Is visible and manageable through the app's governance UI

The hooks section in the UI currently shows "no hooks yet" because it only scans `.orqa/governance/hooks/` which contains shell scripts, not a structured hooks config. Claude hooks in `.claude/settings.json` are invisible to the app.

## Inspiration

The Claude hookify plugin provides a pattern for defining hooks declaratively and translating them to Claude Code's hook format. This could be the bridge between app-managed hooks and CLI compatibility.

## Origin

UAT Round 1 (EPIC-043): F18 — Hooks section shows empty despite Claude hooks existing
