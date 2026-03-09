---
id: IMPL-004
title: "$derived(() => expr) Causes Infinite Loops"
category: implementation
description: >
  Use $derived(expr) for simple expressions or $derived.by(() => expr) for
  function bodies. Never use $derived(() => expr) — it creates a derived
  value that is the function itself, causing infinite re-render loops.
status: active
recurrence: 1
promoted_to: null
tags: [svelte5, runes, reactivity, derived, bug]
---

## What Happened

Using `$derived(() => expression)` instead of `$derived(expression)` or `$derived.by(() => expression)` caused components to enter infinite re-render loops, hanging the app window.

## Why It Was Unexpected

The syntax `$derived(() => ...)` looks valid — it's a function returning a value. But in Svelte 5, `$derived()` expects either a bare expression (`$derived(x + y)`) or uses `$derived.by()` for function bodies. Wrapping a bare expression in an arrow function creates a derived value that IS the function itself, not its return value, triggering unexpected reactivity behavior.

## The Correct Approach

```typescript
// CORRECT — bare expression
let count = $derived(items.length);

// CORRECT — function body with .by()
let filtered = $derived.by(() => items.filter(i => i.active));

// WRONG — causes infinite loop
let count = $derived(() => items.length);
```

## Where This Hit

- `ArtifactMasterDetail.svelte` — `$derived(() => ...)` on artifact list filtering
- `ArtifactNav.svelte` — `$derived(() => ...)` on navigation item computation
- `ArtifactLanding.svelte` — `$derived(() => ...)` on category selection

All three were fixed in the same session by converting to `$derived.by(() => ...)`.

## Prevention

The `svelte5-best-practices` skill documents this pattern. ESLint could potentially catch it with a custom rule.
