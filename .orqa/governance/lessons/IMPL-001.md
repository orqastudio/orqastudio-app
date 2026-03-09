---
id: IMPL-001
title: "Vite Optimize After New Dependencies"
category: dev-environment
description: >
  After installing new npm dependencies, run npx vite optimize before
  starting the dev server to avoid WebView2 white screen on Windows.
status: active
recurrence: 1
promoted_to: null
tags: [dev-environment, vite, windows, dependencies]
---

## Pattern
After `npm install` or when Vite encounters new dependencies for the first time, `cargo tauri dev` launches the app but Vite triggers a dependency optimization reload. On Windows, the Tauri WebView2 webview fails to reconnect after this reload, leaving the app stuck on a white screen. The app only works after a full restart.

## Fix
Run `npx vite optimize` before starting or restarting the dev server whenever new dependencies have been added or on first launch after cloning. This pre-bundles dependencies so Vite skips the runtime optimization reload.

```bash
npx vite optimize
cargo tauri dev
```
