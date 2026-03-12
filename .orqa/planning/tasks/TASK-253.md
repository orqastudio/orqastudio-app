---
id: TASK-253
title: "Move sidecar to sidecars/orqa-sidecar/"
description: "Relocate sidecar/ to sidecars/orqa-sidecar/ and update all references in Makefile and Rust source."
status: todo
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-056
depends-on:
  - TASK-252
acceptance:
  - "sidecar/ moved to sidecars/orqa-sidecar/"
  - "Makefile sidecar targets updated"
  - "sidecar_commands.rs path references updated (5 paths)"
  - "make clippy passes"
  - "make test-rust passes"
---

## What

Move the sidecar directory and update all references.

## How

1. `git mv sidecar sidecars/orqa-sidecar`
2. Update Makefile: `cd sidecar` → `cd sidecars/orqa-sidecar` (3 changes)
3. Update `src-tauri/src/commands/sidecar_commands.rs` (5 path references)
4. Verify with `make clippy && make test-rust`

## Verification

- [ ] `ls sidecars/orqa-sidecar/dist/sidecar.js` succeeds after build
- [ ] `make clippy` passes
- [ ] `make test-rust` passes
