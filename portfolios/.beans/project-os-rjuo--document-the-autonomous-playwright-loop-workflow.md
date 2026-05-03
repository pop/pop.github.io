---
# project-os-rjuo
title: Document the autonomous Playwright loop workflow
status: todo
type: task
priority: normal
created_at: 2026-04-20T16:58:07Z
updated_at: 2026-04-20T16:58:07Z
parent: project-os-q8fy
blocked_by:
    - project-os-0b6n
---

Add a short doc so future agents (and humans) know how to run the e2e loop.

## Context
Epic: `project-os-q8fy`. Depends on the working repro from `project-os-0b6n`.

## Todo
- [ ] Add a `## Testing` section to `CLAUDE.md` describing:
  - One-time setup: `bun install`
  - Running tests: `bun run test:e2e`
  - Interactive debugging: `bun run test:e2e:ui`
  - Where traces land: `playwright-report/`, `test-results/`
  - The agentic loop pattern: edit Rust → `bun run test:e2e` → read failure → repeat
- [ ] If `README.md` exists at repo root, add a brief pointer there; otherwise skip
- [ ] Commit referencing this ticket

## Acceptance
A fresh agent reading `CLAUDE.md` can run the e2e suite without asking clarifying questions.

## Non-goals
- No separate `TESTING.md` file — keep it inline in CLAUDE.md
- No CI docs
