---
id: TASK-429
title: "Enrich graph nodes with status, title, priority as first-class fields"
description: "Update ArtifactNode to expose status, title, description, priority as direct fields instead of requiring frontmatter JSON parsing."
status: todo
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-067
depends-on: []
assignee: null
skills: []
acceptance:
  - "Status, title, description, priority accessible as direct fields on ArtifactNode (not just in frontmatter JSON)"
  - "SDK and frontend can read them without parsing frontmatter"
relationships:
  - target: EPIC-067
    type: delivers
    rationale: "Enriched graph nodes are the foundation for all artifact viewer improvements"
---

## Scope

Update Rust artifact_graph.rs node builder and ArtifactNode struct to promote status, title, description, and priority from frontmatter into first-class fields. Update TypeScript types in the SDK/frontend to match. Update any SDK consumers that read these fields.
