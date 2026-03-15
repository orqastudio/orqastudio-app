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

## How The System Works

### The Graph
- **Nodes** = artifacts (ideas, tasks, rules, decisions, anything)
- **Edges** = typed relationships (the ONLY way things connect)
- **Status** = where each node is in its thought journey
- **Transition rules** = derived from graph state (when connected nodes change, what happens)

### Views Are Graph Queries
- The **roadmap** is the graph filtered to delivery types, grouped by hierarchy, with status as columns
- The **dashboard** is aggregate queries on graph state
- The **artifact viewer** is a single node with its relationships
- The **full graph** is everything

### Two Layers
1. **Core (universal)**: The thinking framework — pillars, ideas, research, rules, lessons, decisions, skills, agents, personas, grounding, wireframes, vision. These types always exist. They represent universal stages of structured thinking.
2. **Delivery (project-configurable)**: The work pipeline — milestones, epics, tasks (or hypotheses, experiments, observations, or whatever the project needs). The user defines what work looks like.

### State Machine
Status transitions are graph queries: "when all nodes connected via `delivers` relationships are in `completed` status → propose transitioning this node to `review`." The state machine isn't a separate system — it's rules about which transitions are valid given the current graph state.

## The Product Philosophy

- **The framework that produces structured outcomes is not optional.** OrqaStudio has a point of view about how thinking should work.
- **Human-led AI.** AI assists and executes. Humans authorise and decide.
- **Clarity before execution.** Most tools optimise for output. OrqaStudio optimises for understanding.
- **Artifact-driven reasoning.** Plans, decisions, and knowledge are living documents, not chat messages.
- **UX-first design.** The UI should be approachable for anyone who thinks in terms of decisions and standards, not terminal commands.

## Current State

This is a Tauri v2 desktop app (Rust backend + Svelte 5 frontend + SQLite). The project is dogfooding — OrqaStudio is being built using OrqaStudio.

See `WORKING-DOCUMENT.md` in the project root for the current architectural discussion about graph-first design, relationship-only connections, and config-driven delivery pipeline.

See `.orqa/documentation/about/vision.md` for the full product vision.
