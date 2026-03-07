![OrqaStudio](ui/lib/assets/banner.png)

# OrqaStudio

OrqaStudio is an AI-assisted clarity engine that helps people turn complex situations into structured understanding and evolving plans.

It applies agile thinking beyond software development, enabling individuals and teams to explore problems, design experiments, and continuously learn through structured retrospection.

---

## Project Philosophy

OrqaStudio focuses on **clarity before execution**. Clear thinking leads to better action.

The platform supports a structured thinking loop:

```
Chaos --> Clarity --> Structured Understanding --> Experiments --> Reflection --> Improved Understanding
```

### Core Principles

- **Clarity before execution** — Clear thinking leads to better action
- **Human-led AI** — AI acts as a structured thinking partner rather than replacing human judgement
- **Artifact-driven reasoning** — Markdown artifacts represent structured knowledge that can evolve over time
- **Reflective learning** — Retrospectives and iteration history enable continuous learning

---

## What OrqaStudio Does

- **Wraps Claude Code** — Same model, same capabilities, but with a UI that surfaces the process layer
- **Backfills governance** — Point it at an existing codebase, answer questions, and OrqaStudio progressively builds the documentation, rules, and agent definitions through conversation
- **Makes process visible** — Scanner dashboards, task pipelines, retrospective cards, and metrics charts replace terminal output
- **Persists everything** — Conversation history linked to artifacts, searchable across sessions

---

## Repository Purpose

This is the main application repository containing the OrqaStudio desktop app source code.

---

## Tech Stack

- **Desktop:** Tauri v2 (Rust backend, lightweight native shell)
- **Frontend:** Svelte 5 (runes, component architecture)
- **AI Integration:** Claude API + Claude Max (via Agent SDK or direct API)
- **Persistence:** SQLite (session history, metrics, project config)
- **Target platforms:** Windows, macOS, Linux

---

## Repository Ecosystem

| Repository | Purpose |
|------------|---------|
| [orqastudio-app](https://github.com/orqastudio/orqastudio-app) | Application source code |
| [orqastudio-brand](https://github.com/orqastudio/orqastudio-brand) | Canonical branding assets and guidelines |
| orqastudio-site | Project website (planned) |
| orqastudio-docs | Public documentation (planned) |

---

## Getting Started

See [Development > Getting Started](docs/development/getting-started.md) for prerequisites and setup instructions.

## Documentation

Documentation lives in the `docs/` directory. Open any `.md` file directly or browse in OrqaStudio's built-in doc viewer.

Documentation is licensed under [CC BY 4.0](docs/LICENSE).

---

## Licensing

Application source code is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

Documentation (`docs/`) is licensed under [Creative Commons Attribution 4.0 (CC BY 4.0)](docs/LICENSE).

Copyright (c) 2026 Bobbi Byrne-Graham

---

## Status

OrqaStudio is currently under active development. APIs and internal structures may change.
