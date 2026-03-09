---
role: artifacts
label: "Tasks"
description: "Scoped work items within an epic."
icon: "check-square"
sort: 3
---

# Tasks

Tasks are scoped work items within an epic. They represent individual units of work that can be assigned to a single agent.

## Lifecycle

```
todo → in-progress → done
```

- **Todo**: Task defined with acceptance criteria
- **In-progress**: Agent assigned and working
- **Done**: Acceptance criteria met and verified

## What Makes a Good Task

- Belongs to a parent epic
- Has clear, testable acceptance criteria
- Can be completed by a single agent in one session
- Does not overlap with other tasks in the same epic

## Frontmatter Schema

Most tasks live as markdown checklist items in their parent epic. A task graduates to a separate `TASK-NNN.md` file when it needs its own detailed tracking — acceptance criteria, agent assignment, scope list, or discussion thread.

```yaml
---
id: TASK-001
title: "Implement session persistence"
status: todo                          # todo | in-progress | done
epic: EPIC-001
created: 2026-03-07
updated: 2026-03-07
assignee: backend-engineer            # Agent name or null
skills: [chunkhound, orqa-repository-pattern, orqa-ipc-patterns]
scope:                                # Files/directories affected
  - src-tauri/src/repo/session_repo.rs
acceptance:                           # What "done" looks like
  - Sessions persist to SQLite
  - Sessions restore on app restart
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
| `skills` | No | string[] | Skills the assignee should load before starting |
| `scope` | No | string[] | Files/directories affected |
| `acceptance` | No | string[] | Acceptance criteria |
| `tags` | No | string[] | Freeform tags |

## The Skills Field

The `skills` field creates a traceability chain from plan to execution:

- **Plan** defines what needs doing
- **Task** specifies who does it (`assignee`) and what knowledge they need (`skills`)
- **Agent** loads those skills before starting
- **Implementation** is done with the right context loaded

When an orchestrator creates a task, it populates `skills` based on the domains the task touches. An agent that picks up the task loads every skill listed before reading any code.

Common skill combinations by domain:

| Domain | Typical Skills |
|--------|---------------|
| Rust backend | `chunkhound`, `orqa-ipc-patterns`, `orqa-repository-pattern`, `rust-async-patterns` |
| Svelte frontend | `chunkhound`, `orqa-store-patterns`, `orqa-ipc-patterns`, `svelte5-best-practices` |
| Streaming pipeline | `chunkhound`, `orqa-streaming`, `orqa-ipc-patterns` |
| Data / SQLite | `chunkhound`, `orqa-repository-pattern`, `orqa-domain-services` |
| Governance / agents | `chunkhound`, `orqa-governance` |

## Related

- Tasks belong to epics in the **Epics** section
- See `docs/product/artifact-framework.md` for the full task schema and lifecycle rules
