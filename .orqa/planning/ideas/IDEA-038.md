---
id: IDEA-038
title: Plugin Distribution via Git Submodules
description: "An official plugins repository under the GitHub org that embeds each plugin as a git submodule. The app reads the repository catalogue, pulls plugins based on user selection, and manages versioning/updates. Each plugin is its own repo with independent releases."
status: captured
created: 2026-03-10
updated: 2026-03-10
pillars:
  - PILLAR-001
research-needed:
  - "Git submodule vs git subtree for plugin embedding — trade-offs for automated updates"
  - "Plugin catalogue format — how does the app discover available plugins from the repo?"
  - "Versioning strategy — semver per plugin repo, pinned in the main catalogue"
  - "Auto-update mechanism — how does the app detect and pull new plugin versions?"
  - "Plugin isolation — how to prevent a broken plugin from affecting the app"
  - "Plugin signing/trust — how to verify plugins from the official repo vs third-party"
promoted-to: null
---
## Motivation

Plugins need a distribution channel that supports independent versioning, safe installation, and automatic updates without shipping everything in the main app binary. Git submodules give us this naturally:

- Each plugin is its own repo → independent versioning, CI, releases
- The official `orqa-plugins` repo embeds them as submodules → single catalogue, atomic snapshots
- The app reads the catalogue and pulls what the user selects → no unused plugins shipped
- Updates are git operations → familiar, auditable, rollback-friendly

## Sketch

### Architecture

```
github.com/orqa-studio/orqa-plugins          (catalogue repo)
  ├── plugins/
  │   ├── governance-dashboard/               (submodule → orqa-studio/plugin-governance-dashboard)
  │   ├── dependency-graph/                   (submodule → orqa-studio/plugin-dependency-graph)
  │   ├── sprint-planning/                    (submodule → orqa-studio/plugin-sprint-planning)
  │   └── ...
  ├── catalogue.json                          (plugin metadata: name, description, version, compatibility)
  └── README.md
```

Each plugin repo:

```
github.com/orqa-studio/plugin-governance-dashboard
  ├── plugin.json                             (manifest: name, version, entry point, SDK version required)
  ├── src/                                    (plugin source — Svelte components, store, etc.)
  ├── test-data/                              (seed data for development/testing)
  └── README.md
```

### App Integration

1. App reads `catalogue.json` from the plugins repo (cached locally, refreshed periodically)
2. User selects plugins to install from a marketplace-style UI
3. App clones the plugin repo into a local plugins directory
4. Plugin is loaded via the SDK — same `artifactGraph` API the built-in views use
5. Updates: app checks plugin repo for new tags, offers to update

### Built-in vs Plugin

The app ships with **default views only** — the artifact browser, viewers, and core navigation. Everything else is a plugin:

| Layer | What | Distribution |
|-------|------|-------------|
| **Built-in** | Artifact browser, viewers (artifact, agent, skill, rule, hook), navigation, conversation | Shipped with app binary |
| **Official plugins** | Governance dashboard, dependency graph, sprint planning, etc. | Official plugins repo |
| **Community plugins** | Third-party extensions | Their own repos, installable via URL |

### Development Workflow

Plugin development uses the `plugin-development` skill (TASK-081), which guides the AI to:

1. Create the plugin in a **new standalone project** (not inside the user's production project)
2. Generate **seed data** for testing (mock `.orqa/` directory with representative artifacts)
3. Develop and test against the seed data using the Artifact Graph SDK
4. Only import into the production project once the plugin is working

This keeps production governance data safe during plugin development.
