# OrqaStudio Architecture Working Document

*Created 2026-03-15 — for holistic discussion across sessions*

---

## The Universal Layer (What OrqaStudio IS)

Everything is a node on a graph. Nodes connect through typed relationships. That's it. The graph is the product.

The app ships with core node types (pillars, ideas, research, rules, lessons, decisions, skills, agents, personas, grounding, wireframes (*COMMENT* Wireframes are software specific artifact - this should be seperated from the core, but still be part of 'discovery'), vision). These types always exist because they represent universal stages of structured thinking. They have statuses that represent stages of thought: captured → exploring → ready → prioritised → active → hold → blocked → review → completed → surpassed → recurring.

The app enforces graph integrity: relationships are bidirectional, nodes have valid statuses, parent-child consistency holds. This is mechanical — the app reads config and enforces it.

## The Project Layer (What the User Is Working On)

The user defines delivery types that represent THEIR work structure. A software project: milestone → epic → task. A research project: hypothesis → experiment → observation. A consulting project: phase → workstream → deliverable.

These delivery types connect to the universal graph through relationships. An epic `delivers` to a milestone. A task `delivers` to an epic. An idea `informs` an epic. A rule `enforces` a decision.

The delivery types have their own status vocabulary IF they need one. Or they can use the universal statuses. The user configures this.

## How Views Work

The roadmap is a view of the graph filtered to delivery types, grouped by the hierarchy the user configured, with status as the column dimension. It's not a separate data structure — it's a graph query rendered as a kanban.

The dashboard is aggregate queries on graph state: how connected is the graph, what's the status distribution, what needs attention (nodes in `review` state), what's improving (trend of governance node count over time).

The artifact viewer is a single node with its relationships visible.

The full graph view is the whole thing.

## Where State Machines Fit

The state machine isn't a separate system. It's a set of rules about which status transitions are valid for which relationship configurations. "A node with incoming `delivers` relationships can move to `review` when all its `delivers` sources are `completed`." That's a graph query that gates a status change.

The state machine config defines:

- Valid statuses (with icons, labels)
- Valid transitions per status
- Auto-rules that are graph queries: "relationship type X, all sources in status Y → transition to Z"

## Open Questions

1. Should delivery types have DIFFERENT statuses than core types? Or is one universal status vocabulary enough?
2. How does the hierarchy depth work? The config defines 3 levels (milestone → epic → task) but what if a project has 2 or 5?
3. Where do ideas and research connect to delivery? An idea gets promoted to an epic — that's a relationship. But the idea stays in Discovery while the epic lives in Delivery. The relationship crosses sections.

## Three Core Principles Being Aligned

1. **Artifacts linked through relationships** — powers the knowledge graph, the ONLY connection mechanism
2. **Systems thinking enforced at app level** — the app's structure teaches and enforces structured thinking
3. **Progress insight via state machine** — users see where things are via status on nodes, transitions driven by graph state

## The Alignment

```
Artifacts exist as nodes
    ↓
Relationships connect them (the ONLY connection mechanism)
    ↓
Each node has a status (where in the thought journey)
    ↓
Transition rules query relationship state
    ("all nodes connected via 'delivers' are completed → move to review")
    ↓
Views (roadmap, dashboard) render graph + state
    ↓
The user sees their thinking structure and its progress
```

Auto-rules as graph queries:

```json
{
  "condition": "all-related-in-status",
  "relationship": "delivers",
  "status": "completed",
  "target": "review"
}
```

## Key Architectural Decisions Made This Session

- **AD-049**: Status represented by icons, colors reserved for artifact types
- **AD-050**: Status transitions are config-driven (project.json)
- **AD-051**: Three-layer configurability — core types hardcoded, instances project-specific, delivery fully configurable
- **IDEA-105**: Delivery pipeline as a future plugin
- **IDEA-106**: Principles/Discovery/Learning section split, wireframes + grounding + personas as first-class artifacts
- **IDEA-107**: App-shipped system docs (conventions) vs project-level rules, required project state machine skill

## Session Artifacts Created

### Epics Completed

- EPIC-064: Enforcement bootstrapping (15 tasks)
- EPIC-073: UAT round 3 (19+ tasks)
- EPIC-074: Dashboard redesign (5 tasks)
- EPIC-075: Documentation reorganisation (6 tasks)
- EPIC-077: Automated status transitions (5 tasks)
- EPIC-078: Configuration-driven delivery pipeline (5 tasks)

### Epics Created (not started)

- EPIC-076: Graph analysis with Cytoscape.js (6 tasks)

### Ideas Captured

- IDEA-095 through IDEA-107 (13 ideas)

### Architecture Decisions

- AD-049 through AD-051

---

## Reconnection Instructions

To restore the full governance system after this holistic discussion:

1. **Restore CLAUDE.md, rules, agents, skills:**
   ```bash
   cp .claude/_backup/CLAUDE.md.bak .claude/CLAUDE.md
   mv .claude/_backup/rules .claude/rules
   mv .claude/_backup/agents .claude/agents
   mv .claude/_backup/skills .claude/skills
   rmdir .claude/_backup
   ```

2. **Or just start a new session:**
   The orqastudio-claude-plugin's `session-start.sh` hook recreates these
   from `.orqa/` source of truth on every session start. The `_backup`
   directory can then be deleted.

3. **Clean up:**
   ```bash
   rm WORKING-DOCUMENT.md  # Delete this file when discussion is complete
   ```
