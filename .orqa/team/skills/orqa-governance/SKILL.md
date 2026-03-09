---
id: orqa-governance
layer: project
title: "Orqa Governance Patterns"
name: orqa-governance
description: |
  OrqaStudio governance patterns: artifact types, scanning pipeline, lesson promotion,
  rule enforcement, frontmatter schemas, and .orqa/ directory structure.
  Use when: Working with governance artifacts (docs, research, lessons, rules),
  modifying scanning or enforcement, or maintaining the .orqa/ directory.
version: 2.0.0
tags: [orqa, governance, artifacts, scanning, lessons, rules, enforcement]
user-invocable: true
---


OrqaStudio's governance layer manages documentation, research, lessons, rules, agents, skills, and hooks as browsable, scannable artifacts. Understanding this system is critical for anyone working on the governance features.

## .orqa/ Directory Structure

```
.orqa/
  project.json              # Project config (name, artifacts array, default model, etc.)
  icon.svg                  # Project icon
  documentation/            # Documentation (tree structure with subdirectories)
    architecture/           #   Architecture docs
    development/            #   Development guides
    process/                #   Process docs
    product/                #   Product docs
    ui/                     #   UI specs
  planning/                 # Planning artifacts
    ideas/                  #   IDEA-NNN.md
    research/               #   Research documents (investigations, designs, spikes)
    milestones/             #   MS-NNN.md
    epics/                  #   EPIC-NNN.md (contain implementation design in body)
    tasks/                  #   TASK-NNN.md
  governance/               # Governance artifacts
    lessons/                #   IMPL-NNN.md
    decisions/              #   AD-NNN.md
    rules/                  #   Rule markdown files
    hooks/                  #   Hook scripts
  team/                     # Team artifacts
    agents/                 #   Agent definitions
    skills/                 #   Skill directories (each has SKILL.md)
```

## Artifact Traceability Chain

```
Task → Epic → Milestone
         ↑
    research-refs → Research
```

- **Tasks** always have `epic:` field referencing an existing EPIC-NNN
- **Epics** always have `milestone:` field referencing an existing MS-NNN
- **Epics** may have `research-refs:` array linking to research documents
- **Research** documents are investigations, design explorations, and spikes
- **There is NO "Plan" artifact type** — epics contain implementation design in their body

### FORBIDDEN

- `plan:` field on any artifact — this field is deprecated and removed
- Creating files in `.orqa/planning/plans/` — this directory no longer exists
- Tasks without an `epic:` field
- Epics without a `milestone:` field

## Config-Driven Artifact Scanning

The `artifacts` array in `project.json` is the SINGLE SOURCE OF TRUTH for what gets scanned and displayed. The scanner does NOT guess — it reads config and scans exactly those paths.

### Config Schema

```jsonc
"artifacts": [
  // Direct type — scans a directory (flat or tree)
  { "key": "docs", "label": "Documentation", "icon": "file-text", "path": ".orqa/documentation" },
  // Group — renders as expandable group, each child scanned independently
  { "key": "planning", "label": "Planning", "icon": "target",
    "children": [
      { "key": "ideas", "label": "Ideas", "path": ".orqa/planning/ideas" },
      { "key": "research", "label": "Research", "path": ".orqa/planning/research" },
      { "key": "epics", "label": "Epics", "path": ".orqa/planning/epics" }
    ]
  }
]
```

### Scanning Behavior (File Explorer Pattern)

The scanner recursively walks directories like a file explorer:

1. **Flat directories** — Scans `.md` files directly (milestones, epics, etc.)
2. **Tree directories** — Recurses into subdirectories, creating `DocNode` entries with `children` for folders
3. **Frontmatter extraction** — Every `.md` file gets YAML frontmatter parsed for `title` (→ label) and `description`
4. **Label priority**: frontmatter `title` > `humanize_name(filename)` > raw filename
5. **Artifact IDs** (all-caps like `EPIC-001`, `AD-015`) are preserved as-is, not humanized
6. **README.md** is navigation metadata, skipped as browsable artifact at all levels
7. **Hidden entries** (`.` or `_` prefix) skipped
8. **Empty directories** omitted from tree

### Critical Rule: Config Paths Must Match Disk

Every `path` in the config must resolve to an actual directory. Moving files on disk requires updating the config. See `.orqa/governance/rules/artifact-config-integrity.md`.

## Artifact Frontmatter Schemas

All governance artifacts use YAML frontmatter parsed by a generic function.

### Epic Frontmatter (key artifact)

```yaml
---
id: EPIC-NNN
layer: project
title: "Epic Title"
status: draft | ready | in-progress | review | done
milestone: MS-NNN
priority: P1 | P2 | P3
research-refs:          # Optional — links to research documents
  - research-doc-name
scoring:
  dogfood-value: 1-5
  foundation: 1-5
  user-visible: 1-5
  scope: 1-5
  dependency-risk: 1-5
score: computed
docs-required: []       # Docs that must exist before implementation
docs-produced: []       # Docs this work creates/updates
depends-on: []
blocks: []
description: >
  What this epic delivers.
tags: []
---

## Implementation Design

[The epic body contains the implementation design that previously lived
in a separate plan document. Data model, IPC contracts, component
breakdown, and approach all go here.]
```

### Task Frontmatter

```yaml
---
id: TASK-NNN
layer: project
title: "Task Title"
status: todo | in-progress | done | surpassed
epic: EPIC-NNN          # REQUIRED — always references an epic
created: YYYY-MM-DD
updated: YYYY-MM-DD
assignee: agent-name
skills: [skill1, skill2]
scope:
  - file/paths
acceptance:
  - criteria
tags: []
---
```

### Research Frontmatter

```yaml
---
title: "Research Title"
status: draft | complete | surpassed
surpassed-by: "reference"   # Set when status: surpassed
category: investigation | design | spike
created: YYYY-MM-DD
updated: YYYY-MM-DD
tags: []
---
```

### Lesson Frontmatter

```yaml
---
id: IMPL-NNN
layer: project
title: "Lesson Title"
category: implementation
recurrence: 0
promoted_to: null
tags: []
---
```

## Artifact Status Workflows

### Epic: `draft → ready → in-progress → review → done`
### Task: `todo → in-progress → done` (or `→ surpassed`)
### Research: `draft → complete → surpassed`
### Decision: `proposed → accepted → superseded` (or `→ deprecated`)
### Idea: `captured → exploring → shaped → promoted` (or `→ archived`)
### Milestone: `planning → active → complete`

## Historical Artifact Preservation

- **Documentation** (`.orqa/documentation/`) — DELETE when outdated, replace with current
- **Research, tasks** — PRESERVE, mark `status: surpassed` with `surpassed-by` reference
- **Never delete** research or task files — they are historical records

## Lesson Pipeline

```
Lesson documented (.orqa/governance/lessons/IMPL-NNN.md)
    → Recurrence tracked (frontmatter count field incremented)
    → Promoted at threshold (recurrence >= 2)
    → Becomes rule or coding standard addition
    → Enforcement verified
```

## Two-Pillar Alignment

Every governance artifact serves at least one pillar:

| Pillar | What It Covers |
|--------|---------------|
| **Clarity Through Structure** | Rules, agents, scanners, enforcement, quality gates, architecture decisions, artifact visibility |
| **Learning Through Reflection** | Lessons, metrics, retrospectives, pattern promotion, knowledge accumulation |

Features that serve neither pillar are out of scope.

## Key Files

| File | Purpose |
|------|---------|
| `.orqa/project.json` | Project configuration (includes `artifacts` array) |
| `.orqa/governance/lessons/` | Implementation lessons (IMPL-NNN.md) |
| `.orqa/governance/decisions/` | Architecture decisions (AD-NNN.md) |
| `.orqa/governance/rules/` | Governance rules |
| `.orqa/governance/hooks/` | Hook scripts |
| `.orqa/planning/ideas/` | Ideas (IDEA-NNN.md) |
| `.orqa/planning/research/` | Research documents (investigations, designs, spikes) |
| `.orqa/planning/milestones/` | Milestones (MS-NNN.md) |
| `.orqa/planning/epics/` | Epics (EPIC-NNN.md) — contain implementation design |
| `.orqa/planning/tasks/` | Tasks (TASK-NNN.md) — always reference an epic |
| `.orqa/team/agents/` | Agent definitions |
| `.orqa/team/skills/` | Skill definitions |
| `.orqa/documentation/` | Documentation tree (subdirs: architecture, product, etc.) |
| `src-tauri/src/domain/artifact.rs` | Frontmatter parsing, artifact types |
| `src-tauri/src/domain/artifact_reader.rs` | Config-driven recursive scanner |
| `src-tauri/src/commands/artifact_commands.rs` | Tree scan and read commands |
| `src-tauri/src/domain/project_settings.rs` | Project settings + ArtifactEntry config types |
