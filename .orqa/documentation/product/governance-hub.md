---
title: "Governance Hub"
description: "How OrqaStudio distributes governance to external AI tools and coexists with their native configurations."
tags: [governance, hub, distribution, coexistence, claude-code, cursor, copilot]
created: 2026-03-09
updated: 2026-03-09
---

# Governance Hub

When a project uses multiple AI tools (Claude Code, Cursor, Copilot, Aider, etc.), each tool has its own configuration format for instructions, rules, and context. Without coordination, these configurations drift apart — rules added in one tool are missing in another, leading to inconsistent behavior.

OrqaStudio can act as a **governance hub** for such projects: a single source of truth for rules, agent instructions, and process standards that distributes governance to each tool in its native format.

**This is a capability, not the product's identity.** OrqaStudio is a clarity engine for structured thinking. The governance hub activates when a project's context calls for it — when multiple AI tools need to share the same standards.

## How It Works

```
.orqa/governance/rules/       ← Single source of truth
    │
    ├── .claude/rules/        ← Symlinks (Claude Code reads these)
    ├── .cursorrules          ← Generated (Cursor reads this)
    ├── .github/copilot-*.md  ← Generated (Copilot reads this)
    └── .aider.conf.yml       ← Generated (Aider reads this)
```

1. **Rules live in `.orqa/`** — One canonical set of governance artifacts
2. **Tool-specific configs are derived** — Either symlinked or generated from the canonical set
3. **Changes flow outward** — Edit in `.orqa/`, tool configs update automatically
4. **Each tool reads its native format** — No tool needs to know about `.orqa/`

## Coexistence Model

OrqaStudio does not replace other AI tools. It coexists with them, adding a governance coordination layer.

| Tool | Native Config | Integration Method |
|------|-------------|-------------------|
| **Claude Code** | `.claude/`, `CLAUDE.md`, `AGENTS.md` | Symlinks: `.claude/` → `.orqa/` |
| **Cursor** | `.cursorrules`, `.cursor/rules/` | Generated from `.orqa/governance/rules/` |
| **GitHub Copilot** | `.github/copilot-instructions.md` | Generated from `.orqa/governance/rules/` |
| **Aider** | `.aider.conf.yml`, `CONVENTIONS.md` | Generated from `.orqa/governance/rules/` |

### Symlink Strategy (Claude Code)

Claude Code is the deepest integration because both tools use the same markdown-based governance format:

```
.claude/rules/    → .orqa/governance/rules/
.claude/agents/   → .orqa/team/agents/
.claude/skills/   → .orqa/team/skills/
.claude/hooks/    → .orqa/governance/hooks/
.claude/CLAUDE.md → .orqa/team/agents/orchestrator.md
```

Both Claude Code and OrqaStudio read the same files. No generation step needed.

### Generation Strategy (Other Tools)

For tools that use different configuration formats, OrqaStudio generates their native config from `.orqa/` content:

1. Read all active rules from `.orqa/governance/rules/`
2. Transform into the tool's expected format
3. Write to the tool's config location
4. Track the generated file so it can be regenerated on changes

## Drift Reconciliation

When governance changes in `.orqa/`, derived configs may be stale. OrqaStudio handles this through:

1. **Manual sync** — User triggers regeneration from the OrqaStudio UI
2. **File watcher** (future) — Detects changes in `.orqa/` and regenerates affected configs
3. **Commit hook** (future) — Regenerates configs as part of the pre-commit check

## Setup Flow

When OrqaStudio is added to a project that already uses AI tools:

1. **Detect** — `project-inference` skill scans for existing tool configurations
2. **Import** — `project-migration` skill reads existing configs and maps them to `.orqa/` artifacts
3. **Deduplicate** — Existing governance that matches canon rules is linked, not duplicated
4. **Link** — Symlinks and generation configs are set up
5. **Verify** — User reviews the imported governance and confirms

See `project-inference`, `project-migration`, and `project-setup` skills for implementation details.

## What the Hub Does NOT Do

- **Replace AI tools** — OrqaStudio adds governance, not AI capabilities
- **Control tool behavior** — Each tool interprets its own config; OrqaStudio ensures the configs are consistent
- **Require all tools** — A project can use OrqaStudio with just one tool, or none (manual governance)
- **Define the product** — The hub is one capability. OrqaStudio's identity is structured thinking, not tool coordination.

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The governance hub makes rules and standards visible and consistent across all tools — governance is no longer scattered across incompatible config files. |
| Learning Through Reflection | When lessons learned in one tool's context become rules, the hub ensures those rules propagate to all tools, preventing the same mistake from recurring in a different context. |
