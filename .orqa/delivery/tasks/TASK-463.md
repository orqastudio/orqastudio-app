---
id: TASK-463
title: "Create grounding documents for all agent roles"
description: "Create 5 concise grounding documents distilled from restructured documentation. Each answers: why this role exists, what good looks like, what goes wrong under pressure. Designed for agent injection, not human browsing."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-064
depends-on:
  - TASK-462
assignee: null
skills:
  - SKILL-037
  - SKILL-011
acceptance:
  - grounding/product-purpose.md created — mission, pillars, identity (30-50 lines)
  - grounding/code-principles.md created — what good code means, architecture boundaries (30-50 lines)
  - grounding/artifact-principles.md created — what good artifacts look like, graph discipline (30-50 lines)
  - grounding/design-principles.md created — UX principles, what good design means (30-50 lines)
  - grounding/research-principles.md created — evidence standards, investigation quality (30-50 lines)
  - Each doc has frontmatter with ID, relationships to source docs, and pillar alignment
  - Content is distilled from restructured docs, not duplicated
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 2 — grounding docs are the foundation for agent purpose injection
  - target: EPIC-064
    type: belongs-to
    rationale: Task belongs to this epic
  - target: TASK-462
    type: informed-by
    rationale: Grounding docs distill from restructured documentation
---

## Scope

Create `.orqa/documentation/grounding/` directory with 5 role-area documents.

Each document answers three questions:
1. **Why does this role exist?** — Connection to mission and pillars
2. **What does "good" look like?** — The principles that define quality for this role
3. **What goes wrong under pressure?** — Specific failure modes and how to recognize them

### Documents

| File | Grounds | Distilled From |
|------|---------|---------------|
| product-purpose.md | Orchestrator, Planner, Writer | DOC-046 (vision), DOC-039 (governance), pillars |
| code-principles.md | Implementer, Reviewer | DOC-021 (coding-standards), architecture decisions |
| artifact-principles.md | Orchestrator, Writer, Researcher, Governance Steward | DOC-036 (artifact-framework), DOC-039 (governance) |
| design-principles.md | Designer | DOC-049 (design-system), DOC-053 (interaction-patterns), DOC-043 (personas) |
| research-principles.md | Researcher | Research methodology skill content |

### Constraints

- Maximum 50 lines per document — these are injected into agent context, not browsed
- No procedural content — grounding is about purpose and principles, not how-to
- Content must be distilled, not duplicated — reference the source docs for detail
