---
title: "Artifact Framework"
category: product
tags: [artifacts, governance, structured-thinking]
created: 2026-03-07
updated: 2026-03-07
---

# Artifact Framework

**Date:** 2026-03-07

> OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

This document defines the artifact types, schemas, connections, and lifecycle that form OrqaStudio's structured thinking layer. All artifacts are markdown documents with YAML frontmatter, stored in `.orqa/` and browsable in the UI.

---

## Design Principles

### Markdown-First

Artifacts are markdown documents. The underlying UX model is **browsable markdown with structured frontmatter**. Every artifact must be readable and useful in its raw form — in OrqaStudio's UI, in a text editor, or on GitHub.

Richer views (kanban boards, dashboards, graph visualizations, priority matrices) are **optional layers built on top** of the markdown. They project different views of the same underlying data. The markdown is the source of truth.

### Frontmatter as Structured Data

YAML frontmatter provides machine-parseable metadata that enables:

- **Indexing** — OrqaStudio can parse frontmatter into SQLite for search, filtering, and aggregation
- **Connections** — Fields like `milestone`, `epic`, `depends-on` create a navigable web of relationships
- **Status tracking** — Workflow states are frontmatter fields, not separate databases
- **Priority scoring** — Dimension scores in frontmatter feed the prioritisation framework

### Documentation-First Gate

Documentation is not an afterthought — it is a **gate for work**. Every epic declares:

- **`docs-required`** — what documentation must exist and be approved before implementation begins
- **`docs-produced`** — what documentation this work will create or update on completion

This creates an automated traceability chain: research validates ideas, plans spec epics, implementation produces architecture decisions and doc updates. At every stage, the system records what was decided and why, so you can trace back to understand how you got here and inform what comes next.

This is a key differentiator of OrqaStudio as a platform. Projects built with OrqaStudio self-document their decision history through the artifact framework. The documentation isn't separate from the work — it IS part of the work.

### Deadlines

Some projects operate under time constraints. The artifact framework supports optional deadlines on milestones and epics. When a deadline is present:

- It adds **urgency context** to prioritisation — a P2 item with an imminent deadline should be treated as effectively P1
- It enables **time-based views** — timeline projections, deadline warnings, schedule risk analysis
- It remains **optional** — projects without hard deadlines (like conceptual or research projects) simply omit the field

Deadlines are a reality of commercial practice. The framework accommodates them without requiring them.

### Progressive Enhancement

The same artifact works at three levels of sophistication:

1. **File on disk** — readable in any text editor, discoverable with `find` or `grep`
2. **Browsable in UI** — OrqaStudio renders the markdown with syntax highlighting and navigation
3. **Interactive views** — dashboards, boards, and graphs project frontmatter data into richer interfaces

Each level adds capability without replacing the previous one. A user who never opens the interactive views still has full access to all knowledge through the markdown.

---

## Artifact Types

### Overview

| Type | ID Pattern | Directory | Purpose |
|------|-----------|-----------|---------|
| **Milestone** | `MS-NNN` | `.orqa/milestones/` | Strategic goal with gate question |
| **Epic** | `EPIC-NNN` | `.orqa/epics/` | Trackable work unit within a milestone |
| **Task** | `TASK-NNN` | `.orqa/tasks/` | Individual implementation unit within an epic |
| **Idea** | `IDEA-NNN` | `.orqa/ideas/` | Candidate for future work, needs validation |
| **Plan** | (filename) | `.orqa/plans/` | Design document referenced by epics |
| **Lesson** | `IMPL-NNN` | `.orqa/lessons/` | Learning capture from implementation |
| **Research** | (filename) | `.orqa/research/` | Investigation artifact producing decisions |
| **Decision** | `AD-NNN` | `.orqa/decisions/` | Architecture decision record — captures what was decided and why |

Plans, Lessons, and Research already have established schemas (see their respective READMEs). This document defines the schemas for Milestones, Epics, Tasks, and Ideas, and the connections between all types.

### Connections

```
Milestone
  └── Epic (milestone: MS-NNN)
        ├── Task (epic: EPIC-NNN)  — inline checklist or separate file
        └── Plan (plan: filename)  — design document
              └── Research (research-refs: [])

Idea ──promote──> Epic (when validated)

Lesson ──promote──> Rule / Skill / Coding Standard

Research ──promote──> Decision (AD-NNN)

Decision ──supersedes──> Decision (when updated)
```

---

## Schemas

### Milestone (`MS-NNN`)

Milestones are strategic goals. Each has a gate question that determines when the milestone is complete.

```yaml
---
id: MS-001
title: "Dogfooding"
status: active                    # planning | active | complete
created: 2026-03-07
updated: 2026-03-07
deadline: null                    # ISO date or null — optional time constraint
gate: "Can we use this app instead of the terminal for governance management, conversation debugging, and structured thinking about the project?"
description: >
  OrqaStudio is usable as a daily workspace for building OrqaStudio itself.
epic-count: 10                    # Total epics in this milestone
completed-epics: 0                # Epics with status: done
tags: []
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `MS-NNN` identifier |
| `title` | Yes | string | Human-readable milestone name |
| `status` | Yes | enum | `planning`, `active`, `complete` |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `deadline` | No | date/null | ISO date for time-constrained milestones, null otherwise |
| `gate` | Yes | string | Question that determines completion |
| `description` | Yes | string | What this milestone achieves |
| `epic-count` | No | integer | Total epics belonging to this milestone |
| `completed-epics` | No | integer | Epics with `status: done` |
| `tags` | No | string[] | Freeform tags |

### Epic (`EPIC-NNN`)

Epics are trackable work units. Each belongs to a milestone and may reference a plan (design document) and contain tasks.

```yaml
---
id: EPIC-001
title: "AI Transparency Wiring"
status: draft                     # draft | ready | in-progress | review | done
priority: P1                     # P1 | P2 | P3
milestone: MS-001
created: 2026-03-07
updated: 2026-03-07
deadline: null                    # ISO date or null — optional time constraint
plan: null                        # Plan filename (without .md) or null
depends-on: []                    # Other epic IDs this depends on
blocks: []                        # Epic IDs this blocks
assignee: null                    # Agent name or null
pillar:                           # Product pillars served
  - clarity-through-structure
scoring:                          # Priority dimension scores
  pillar: 5
  impact: 5
  dependency: 3
  effort: 2
score: 17.5                      # Computed: (pillar*3 + impact*2 + dependency*3) / effort
roadmap-ref: "D1"                 # Reference to roadmap item
docs-required:                    # Documentation that must exist before work begins
  - docs/architecture/streaming-pipeline.md
docs-produced:                    # Documentation this work creates or updates
  - docs/architecture/streaming-pipeline.md (update with new events)
tags: [streaming, transparency]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `EPIC-NNN` identifier |
| `title` | Yes | string | Human-readable epic title |
| `status` | Yes | enum | `draft`, `ready`, `in-progress`, `review`, `done` |
| `priority` | Yes | enum | `P1`, `P2`, `P3` — derived from score |
| `milestone` | Yes | string | Milestone ID this belongs to |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `deadline` | No | date/null | ISO date for time-constrained epics, null otherwise |
| `plan` | No | string | Plan filename (without `.md`) in `.orqa/plans/` |
| `depends-on` | No | string[] | Epic IDs this depends on |
| `blocks` | No | string[] | Epic IDs or milestone IDs this blocks |
| `assignee` | No | string | Agent name or role |
| `pillar` | No | string[] | `clarity-through-structure`, `learning-through-reflection` |
| `scoring` | No | object | Dimension scores for priority calculation |
| `score` | No | number | Computed priority score |
| `roadmap-ref` | No | string | Reference to roadmap section |
| `docs-required` | No | string[] | Documentation that must exist before work begins |
| `docs-produced` | No | string[] | Documentation this work will create or update |
| `tags` | No | string[] | Freeform tags |

**Task checklists in epics:** Tasks are listed as markdown checklists in the epic body. When a task needs its own detailed tracking (acceptance criteria, agent assignment, discussion), it graduates to a separate `TASK-NNN.md` file in `.orqa/tasks/`.

### Task (`TASK-NNN`)

Tasks are individual implementation units. Most tasks live as checklist items in their parent epic. Separate task files are created only when the task needs its own detailed tracking.

```yaml
---
id: TASK-001
title: "Emit SystemPromptSent event from stream_commands.rs"
status: todo                      # todo | in-progress | done
epic: EPIC-001
created: 2026-03-07
updated: 2026-03-07
assignee: backend-engineer
scope:                            # Files/directories affected
  - src-tauri/src/commands/stream_commands.rs
acceptance:                       # What "done" looks like
  - SystemPromptSent event emitted before sidecar call
  - Event carries custom_prompt and governance_prompt
  - Frontend receives and displays the event
tags: []
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `TASK-NNN` identifier |
| `title` | Yes | string | Concise task description |
| `status` | Yes | enum | `todo`, `in-progress`, `done` |
| `epic` | Yes | string | Parent epic ID |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `assignee` | No | string | Agent name |
| `scope` | No | string[] | Files/directories affected |
| `acceptance` | No | string[] | Acceptance criteria |
| `tags` | No | string[] | Freeform tags |

### Idea (`IDEA-NNN`)

Ideas are candidates for future work. They need research, validation, and shaping before becoming epics. Each idea captures the concept, its alignment with the vision, and what investigation is needed.

```yaml
---
id: IDEA-001
title: "Multi-Provider Ecosystem"
status: captured                  # captured | exploring | shaped | promoted | archived
created: 2026-03-07
updated: 2026-03-07
pillar:
  - clarity-through-structure
promoted-to: null                 # Epic ID if promoted, null otherwise
research-needed:                  # What needs investigating before this becomes an epic
  - Provider SDK compatibility assessment
  - Cost model research
  - UX for provider switching
docs-produced:                    # Documentation this idea will produce when explored
  - .orqa/research/ (research artifact)
  - .orqa/plans/ (implementation plan if promoted)
tags: [providers, composability]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `IDEA-NNN` identifier |
| `title` | Yes | string | Human-readable idea name |
| `status` | Yes | enum | `captured`, `exploring`, `shaped`, `promoted`, `archived` |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `pillar` | No | string[] | Product pillars served |
| `promoted-to` | No | string | Epic ID if promoted, null otherwise |
| `research-needed` | No | string[] | Questions to answer before promotion |
| `docs-produced` | No | string[] | Documentation this idea will produce when explored |
| `tags` | No | string[] | Freeform tags |

### Decision (`AD-NNN`)

Decisions are architecture decision records. Each captures a significant technical or design choice that constrains future work — what was decided, why, and what the consequences are. Decisions are produced by research and supersede earlier decisions when the situation changes.

```yaml
---
id: AD-001
title: "Use Tauri Channel<T> for streaming IPC"
status: accepted                  # proposed | accepted | superseded | deprecated
created: 2026-03-07
updated: 2026-03-07
category: ipc                     # ipc | data | ui | security | tooling | process
research-refs:                    # Research artifacts that produced this decision
  - .orqa/research/mvp/streaming-ipc.md
supersedes: null                  # AD-NNN of the decision this replaces, or null
superseded-by: null               # AD-NNN of the decision that replaced this, or null
tags: [streaming, ipc, tauri]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `AD-NNN` identifier |
| `title` | Yes | string | Human-readable decision title |
| `status` | Yes | enum | `proposed`, `accepted`, `superseded`, `deprecated` |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `category` | Yes | string | Broad category: `ipc`, `data`, `ui`, `security`, `tooling`, `process` |
| `research-refs` | No | string[] | Paths to research artifacts that informed this decision |
| `supersedes` | No | string | `AD-NNN` of the decision this replaces, or null |
| `superseded-by` | No | string | `AD-NNN` of the decision that replaced this, or null |
| `tags` | No | string[] | Freeform tags |

The decision body follows the standard structure: **Context** (what situation prompted this decision), **Decision** (what was chosen and why), **Consequences** (what becomes easier, harder, or constrained).

---

## Status Workflows

These are defaults. OrqaStudio's long-term goal is configurable status workflows per project.

### Milestone

```
planning ──> active ──> complete
```

### Epic

```
draft ──> ready ──> in-progress ──> review ──> done
```

### Task

```
todo ──> in-progress ──> done
```

### Idea

```
captured ──> exploring ──> shaped ──> promoted ──> (becomes Epic)
                                  └──> archived    (rejected or deferred)
```

### Plan (existing)

```
draft ──> approved ──> in-progress ──> complete
```

### Lesson (existing)

```
active ──> promoted (recurrence >= threshold)
```

### Decision

```
proposed ──> accepted ──> superseded
                      └──> deprecated
```

---

## Prioritisation Framework

### Dimensions

Priority is computed from scored dimensions. Dimensions are configurable per project via `.orqa/project.json`.

**Default dimensions for OrqaStudio:**

| Dimension | Scale | Weight | What It Measures |
|-----------|-------|--------|-----------------|
| `pillar` | 0-5 | x3 | Alignment with product pillars. 0=neither, 3=one pillar, 5=both. |
| `impact` | 1-5 | x2 | How much this changes the daily user experience. |
| `dependency` | 1-3 | x3 | How many other items are blocked. 1=independent, 3=critical path. |
| `effort` | 1-5 | divisor | Implementation size. Higher = lower score. |

### Scoring Formula

```
score = (pillar * 3 + impact * 2 + dependency * 3) / effort
```

### Priority Bands

| Band | Score Range | Meaning |
|------|-------------|---------|
| **P1** | >= 10.0 | Critical path — milestone blocked without this |
| **P2** | 5.0 - 9.9 | High value — significant quality improvement |
| **P3** | < 5.0 | Nice to have — improves experience but not blocking |

### Configuration

Priority dimensions, weights, and bands are stored in `.orqa/project.json` under the `priority` key. Different projects can define different dimensions and weights. The formula structure (weighted sum with divisor dimension) is fixed; the dimensions and their weights are configurable.

---

## Directory Structure

```
.orqa/
├── project.json              # Project config including priority dimensions
├── milestones/
│   ├── MS-001.md             # Dogfooding
│   └── MS-002.md             # MVP
├── epics/
│   ├── EPIC-001.md           # AI Transparency Wiring
│   ├── EPIC-002.md           # Settings UI
│   └── ...
├── tasks/                    # Only for tasks that need separate files
│   └── TASK-001.md
├── ideas/
│   ├── IDEA-001.md           # Multi-Provider Ecosystem
│   └── ...
├── plans/                    # Already exists
│   ├── README.md
│   └── composability-gate.md
├── lessons/                  # Already exists
│   ├── IMPL-001.md
│   └── ...
├── decisions/                # Architecture decision records
│   └── AD-001.md
├── research/                 # Already exists
│   ├── README.md
│   └── mvp/
└── icon.svg
```

---

## ID Assignment

All artifact IDs auto-increment within their type:

- `MS-001`, `MS-002`, ...
- `EPIC-001`, `EPIC-002`, ...
- `TASK-001`, `TASK-002`, ...
- `IDEA-001`, `IDEA-002`, ...
- `IMPL-001`, `IMPL-002`, ... (existing)
- `AD-001`, `AD-002`, ...

IDs are stable — never reused after deletion. The next ID is determined by scanning existing files in the directory.

---

## Artifact Visibility in OrqaStudio

### Core UX Principle

**Markdown documents browsable in the UI is the underlying UX model.** Every `.orqa/` artifact is rendered as a readable document within OrqaStudio's artifact browser. This is the foundational layer — all other interaction modes are optional enhancements built on top.

### Visibility Layers

| Layer | What It Provides | When Built |
|-------|-----------------|------------|
| **Document browser** | Navigate and read `.orqa/` artifacts as rendered markdown | Dogfooding (EPIC-005) |
| **Frontmatter sidebar** | Structured metadata displayed alongside the document | Dogfooding (EPIC-005) |
| **Status filtering** | Filter artifacts by status, priority, milestone, type | MVP |
| **Board views** | Kanban-style board projecting epic/task status | MVP |
| **Priority dashboard** | Scored and ranked backlog with band indicators | MVP |
| **Graph views** | Dependency and connection visualisation | Post-MVP |

### What the Browser Shows

For each artifact type, the browser renders:

- **Title and ID** from frontmatter
- **Status badge** with colour coding
- **Priority band** (P1/P2/P3) for epics
- **Milestone membership** for epics
- **Connection links** — clickable references to related artifacts
- **Full markdown body** — the document content, rendered with syntax highlighting

---

## Traceability Web

The frontmatter fields create a navigable web across all artifact types:

```
Milestone ←── milestone ── Epic ←── epic ── Task
                              │
                              ├── plan ──→ Plan ←── research-refs ── Research
                              │
                              ├── depends-on / blocks ──→ Other Epics
                              │
                              ├── docs-required ──→ Existing Docs (gate)
                              │
                              ├── docs-produced ──→ New/Updated Docs (output)
                              │
                              └── pillar ──→ Product Vision

Idea ──research-needed──→ Research ──→ (validates) ──→ promoted-to ──→ Epic
                                    └── docs-produced ──→ Research artifact + Plan

Lesson ──promoted-to──→ Rule / Skill

Research ──produces──→ Decision (AD-NNN) ──supersedes──→ Earlier Decision
```

### Documentation Traceability Chain

Every stage of work self-documents its decisions:

```
Idea captured
  → Research investigates (produces .orqa/research/ artifact)
  → Research resolves → Architecture Decision recorded (.orqa/decisions/AD-NNN.md, indexed in docs/architecture/decisions.md)
  → Idea promoted → Epic created (references plan, lists docs-required)
  → Plan written and approved (docs-required gate satisfied)
  → Implementation begins (produces code + docs-produced artifacts)
  → Completion updates architecture docs, component specs, schemas
  → Lessons captured (.orqa/lessons/) → promoted to rules/skills
```

At any point, you can trace backwards: "Why does this rule exist?" → lesson → epic → plan → research → original idea. This is the automated documentation process — the system records what was decided at each stage and why.

This enables:
- **Impact analysis** — "What breaks if this epic is delayed?" (follow `blocks` links)
- **Progress tracking** — "How far along is Milestone 1?" (count epic statuses)
- **Decision traceability** — "Why did we build this?" (epic → plan → research → decision)
- **Learning loops** — "What did we learn?" (lesson → promoted rule)
- **Documentation readiness** — "Can we start this work?" (check `docs-required` gate)
- **Documentation completeness** — "Did we update all the docs?" (check `docs-produced` output)

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The artifact framework makes thinking visible and structured. Every idea, plan, and work unit is a first-class, browsable, connected document — not hidden in terminal output or chat history. |
| Learning Through Reflection | Ideas, lessons, and research feed the learning loop. Promotion pipelines (idea→epic, lesson→rule, research→decision) ensure knowledge compounds over time. |
