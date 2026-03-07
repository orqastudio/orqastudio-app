---
title: "Product Vision"
category: product
tags: []
created: 2026-03-02
updated: 2026-03-07
---

# Product Vision

**Date:** 2026-03-07

## Vision

OrqaStudio is an AI-assisted clarity engine designed to help people turn messy situations into structured understanding and evolving plans.

Rather than focusing purely on task management or software generation, OrqaStudio focuses on improving the quality of thinking that leads to action.

The platform operationalises agile thinking through AI-assisted reasoning, enabling individuals and teams to explore problems, shape ideas, design experiments, and continuously learn through structured retrospection.

## Core Principles

**Clarity before execution** — Clear thinking leads to better action.

**Human-led AI** — AI acts as a structured thinking partner rather than replacing human judgement.

**Artifact-driven reasoning** — Markdown artifacts represent structured knowledge that can evolve over time.

**Reflective learning** — Retrospectives and iteration history enable continuous learning.

## Agile Learning Loop

The core cycle that OrqaStudio operationalises:

```
Chaos --> Structured Understanding --> Experiments / Backlog --> Execution --> Retrospective --> Improved Understanding
```

Every feature in OrqaStudio serves this loop. The system helps users move from ambiguity to clarity, from clarity to action, and from action to reflection that feeds the next cycle.

---

## Problem

Agentic coding tools like Claude Code are powerful, but they operate as developer-facing CLI tools with no product management layer. The governance framework that makes agentic development reliable — agents, skills, rules, learning loops, documentation-first workflow — lives in dotfiles, markdown documents, and terminal output. There is no tool that lets a Product Manager or Tech Lead define the process, delegate implementation to AI agents, and verify the results through a managed cycle.

The result: AI-assisted development produces inconsistent quality, accumulates technical debt invisibly, and requires deep developer expertise to manage. Product managers are locked out of the implementation loop entirely, and solo technical PMs cannot leverage agentic tools without becoming full-time developers.

## Solution

OrqaStudio is a desktop application that wraps the Claude Code CLI with a visual product management layer. Governance artifacts — agents, skills, rules, architecture decisions, learning loops — are native `.claude/` files on disk, the same format that Claude Code CLI reads and writes. OrqaStudio adds a UI where these artifacts live alongside the conversation as interactive, editable documents. The system learns from every session and feeds improvements back into the governance framework automatically.

Users can switch between OrqaStudio and the Claude Code CLI interchangeably. Both tools operate on the same `.claude/` directory. OrqaStudio does not replace the CLI — it makes the CLI's file-based governance visible and manageable through a graphical interface.

OrqaStudio turns the invisible infrastructure of managed agentic development into a tangible, manageable product.

## Primary Users

### Product Managers & Tech Leads (Primary)

Technical product managers and tech leads who use Claude Code agents and want to:

- Define product requirements, architecture, and standards through a structured UI
- Delegate implementation to AI agents with confidence that process governance is enforced
- Review and approve implementation plans, tool calls, and code changes through a visual interface
- Track quality metrics, scanner results, and learning loop outcomes over time
- Manage the full product lifecycle — from research through implementation to verification — without needing dedicated developer resource for every task

A capable solo technical PM should be able to use OrqaStudio to build well-researched, well-considered products by defining the governance framework, delegating to agents, and reviewing results — while retaining architecture oversight and code review authority.

### Developers (Secondary)

Developers who already use Claude Code CLI and want structured, repeatable, improving processes for their AI-assisted work. OrqaStudio makes the CLI's governance layer visible and manageable rather than buried in dotfiles and terminal output. Developers can use OrqaStudio and the CLI interchangeably — the same `.claude/` artifacts power both.

### The Key Insight

OrqaStudio is not a developer tool that happens to have process features. It is a **product management tool** that automates the implementation cycle. The difference matters: the UI, the workflow, and the default experience are designed for someone who thinks in terms of product requirements, architecture decisions, and quality standards — not someone who thinks in terms of code editors and terminal commands.

## Product Pillars

### Pillar 1: Self-Learning Loop

The system improves over time. Every session contributes to a growing body of knowledge:

- **Lesson capture** — Implementation failures are documented as IMPL entries with recurrence tracking
- **Pattern promotion** — When a lesson recurs, it is automatically promoted to a rule, coding standard, or skill update
- **Retrospectives** — Process-level failures become RETRO entries that inform future governance evolution
- **Metrics tracking** — Pass/fail rates, coverage trends, violation recurrence are visualized over time
- **Session continuity** — Handoff notes and searchable session history prevent context loss between sessions

The learning loop is the core differentiator. A team using OrqaStudio for a month will have a fundamentally better governance framework than when they started — without manually writing any of it.

### Pillar 2: Process Governance

Standards, rules, and workflows are visible, enforceable, and manageable:

- **Rule enforcement** — Rules are not just documents; they are active constraints that agents follow and the UI surfaces
- **Agent management** — Agent definitions, skills, and tool assignments are browsable and editable
- **Scanner dashboards** — Quality scanners run automatically and their results are visualized as pass/fail trends
- **Architecture decisions** — AD records are tracked, cross-referenced, and compliance is verified in plans
- **Documentation-first workflow** — The system enforces document -> approve -> implement -> verify

Governance is not a document collecting dust. It is a living, enforceable, visible layer of the development process.

## Dogfooding Principle

OrqaStudio is built using OrqaStudio alongside the Claude Code CLI. Once the MVP delivers a working conversation UI with Claude integration, the project transitions from terminal-only governance (reading raw `.claude/` files and CLI output) to using OrqaStudio's UI as the primary governance management layer — while the CLI remains available for all development tasks.

This is not optional — it is a foundational design constraint:

- **Every governance feature must be good enough for OrqaStudio's own team to use daily.** If a feature isn't useful for managing this project, it isn't useful for anyone.
- **Deficiencies discovered through self-use are highest-priority fixes.** The dogfooding loop is the primary driver of roadmap priority after the MVP.
- **OrqaStudio and the CLI coexist permanently.** The transition is from "invisible CLI governance buried in dotfiles" to "visible governance through OrqaStudio's UI." The CLI continues to work against the same `.claude/` files. See [Product Governance](/product/governance) for transition criteria.

## CLI Interoperability

OrqaStudio is a companion to the Claude Code CLI, not a replacement for it. This interoperability is a foundational design constraint:

- **Shared artifact format** — All governance artifacts OrqaStudio creates (`.claude/rules/`, `.claude/hooks/`, `.claude/agents/`, `.claude/skills/`, `CLAUDE.md`) are native Claude Code artifacts. They work identically whether accessed through OrqaStudio's UI or the CLI.
- **Bidirectional editing** — Users can edit `.claude/` files in OrqaStudio's artifact editor, in a text editor, or through Claude Code CLI sessions. OrqaStudio's file watcher detects external changes and reflects them in the UI.
- **No lock-in** — A user can stop using OrqaStudio at any time and continue with the CLI alone. All governance artifacts remain functional on disk.
- **SQLite is a derived cache** — OrqaStudio's SQLite database stores session history, project metadata, and indexed artifact data. The `.claude/` files on disk are the source of truth for governance. If the database is deleted, OrqaStudio re-indexes from disk on next launch.
- **CLI detection** — OrqaStudio checks for Claude Code CLI availability at startup and surfaces its status in the UI. The CLI is a prerequisite for AI-powered features.

## Key Differentiators

1. **Clarity engine, not code generator** — Designed to improve the quality of thinking that leads to action, not just generate output
2. **Product management, not developer tooling** — Designed for PMs and tech leads who define process and review results, not just developers who write code
3. **Native Claude Code format** — All governance artifacts are standard `.claude/` files that work identically in OrqaStudio and the Claude Code CLI. No proprietary formats, no lock-in.
4. **Process visibility** — What was invisible in the CLI (governance artifacts, scanner results, learning loops) becomes a first-class UI
5. **Automated governance backfill** — Point at an existing codebase, answer questions, and OrqaStudio builds the governance framework through conversation
6. **Continuous improvement** — The system genuinely gets smarter over time through the learning loop, not just accumulating conversation history
7. **Solo PM capability** — A technical PM can define product standards, delegate to Claude Code agents, and ship software with architecture oversight but without dedicated developer resource for every task
8. **Dogfooding-driven design** — OrqaStudio is its own first customer, ensuring every feature is validated by real use before release
