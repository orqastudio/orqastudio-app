# OrqaStudio

## What It Is

OrqaStudio is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans.

It is not a task tracker. It is not a code editor. It is cognitive infrastructure for structured reasoning and action.

## The Core Idea

**Everything is a node on a graph. Nodes connect through typed relationships. The graph IS the thinking made visible.**

When a user connects an idea to a pillar, or a task to an epic, they're not filling in metadata — they're making their thinking visible and structured. The relationships aren't data; they ARE the structured thinking.

## Three Principles

### 1. Clarity Through Structure
Making thinking visible. If it's not structured and browsable, it doesn't exist yet. Governance artifacts, decisions, plans, and knowledge are first-class visible things — not hidden config files or terminal output.

### 2. Learning Through Reflection
The system improves over time. Mistakes are documented, patterns are extracted, and governance evolves. Every cycle produces not just output but insight that feeds the next cycle.

### 3. Purpose Through Continuity
The user's original intent survives implementation pressure. When scope drifts, when decisions are lost between sessions, when execution diverges from intent — the system surfaces that drift before it compounds.

## Three-Layer Artifact Model

### Layer 1: App-Fixed (Platform)
Definitions that ship with OrqaStudio. Immutable by projects. Defines the canonical types, relationships, and statuses.
- **Location:** `.orqa/documentation/platform/`

### Layer 2: App-Required (Project Foundation)
Artifacts every project must have. Editable content, but existence enforced.
- **Pillars:** `.orqa/principles/pillars/`
- **Vision:** `.orqa/principles/vision/`
- **Personas:** `.orqa/principles/personas/`
- **Grounding:** `.orqa/principles/grounding/`

### Layer 3: Project-Scoped
All other artifacts created during the project's lifecycle.
- **Discovery:** `.orqa/discovery/` (ideas, research, wireframes)
- **Delivery:** `.orqa/delivery/` (milestones, epics, tasks)
- **Process:** `.orqa/process/` (decisions, rules, lessons, skills, agents)
- **Documentation:** `.orqa/documentation/project/`

## Canonical Relationship Vocabulary

All artifact connections use the `relationships` frontmatter array. No standalone connection fields.

| Forward | Inverse | Semantic |
|---------|---------|----------|
| `informs` | `informed-by` | Knowledge flows downstream |
| `evolves-into` | `evolves-from` | Artifact lineage |
| `drives` | `driven-by` | Decision motivates work |
| `governs` | `governed-by` | Decision governs standards |
| `delivers` | `delivered-by` | Work delivers to parent |
| `enforces` | `enforced-by` | Rule enforces decision |
| `grounded` | `grounded-by` | Artifact anchored to principle |
| `observes` | `observed-by` | Agent watches artifact |
| `merged-into` | `merged-from` | Artifact consolidation |
| `synchronised-with` | `synchronised-with` | Paired content (self-inverse) |

Project relationships (e.g. `depends-on`/`depended-on-by`) are defined in `project.json`.

## 12 Canonical Statuses

`captured` → `exploring` → `ready` → `prioritised` → `active` → `hold` / `blocked` → `review` → `completed` → `surpassed` / `archived` / `recurring`

## Four Enforcement Layers

1. **App-layer** — Rust integrity checks validate graph state
2. **Integrity scanner** — Detects broken links, missing inverses, vocabulary violations
3. **Status transitions** — Graph-query-based auto-transitions
4. **Git hooks** — Pre-commit validation via plugin hooks

## Navigation = Graph Filters

Nav sections in `project.json` are views into the graph, not filesystem directories:
- **Principles** → pillars, vision, personas, grounding
- **Discovery** → ideas, research, wireframes, decisions
- **Learning** → rules, lessons, skills, agents
- **Delivery** → milestones, epics, tasks
- **Documentation** → platform docs, project docs

## How The System Works

### The Graph
- **Nodes** = artifacts (ideas, tasks, rules, decisions, anything)
- **Edges** = typed relationships (the ONLY way things connect — no standalone fields)
- **Status** = where each node is in its thought journey
- **Transition rules** = derived from graph state (when connected nodes change, what happens)

### Views Are Graph Queries
- The **roadmap** is the graph filtered to delivery types, grouped by `delivers` hierarchy, with status as columns
- The **dashboard** is aggregate queries on graph state
- The **artifact viewer** is a single node with its relationships
- The **full graph** is everything

### State Machine
Status transitions are graph queries: "when all nodes connected via `delivers` relationships are in `completed` status → propose transitioning this node to `review`."

## The Product Philosophy

- **The framework that produces structured outcomes is not optional.** OrqaStudio has a point of view about how thinking should work.
- **Content IS the platform.** Artifacts are the product — their content, structure, and relationships define the system.
- **Human-led AI.** AI assists and executes. Humans authorise and decide.
- **Clarity before execution.** Most tools optimise for output. OrqaStudio optimises for understanding.
- **Artifact-driven reasoning.** Plans, decisions, and knowledge are living documents, not chat messages.
- **UX-first design.** The UI should be approachable for anyone who thinks in terms of decisions and standards, not terminal commands.

## Current State

This is a Tauri v2 desktop app (Rust backend + Svelte 5 frontend + SQLite). The project is dogfooding — OrqaStudio is being built using OrqaStudio.

**Graph-first migration (EPIC-079) is complete for the app codebase.** All 1058 artifacts migrated, Rust backend and Svelte frontend use canonical vocabulary only, 623 tests pass. `.orqa-backup/` is preserved for review — do not delete yet.

**Next: Library migration in progress.** The external libraries (`@orqastudio/types`, `@orqastudio/integrity-validator`, `@orqastudio/sdk`) still use old vocabulary and are causing false integrity errors in the running app. Dev publish workflows (`publish-dev.yml`) have been added to all three repos but not yet pushed. See memory file `project_library_migration_state.md` for full migration plan and dependency order.

See `.orqa/principles/vision/vision.md` for the full product vision.
