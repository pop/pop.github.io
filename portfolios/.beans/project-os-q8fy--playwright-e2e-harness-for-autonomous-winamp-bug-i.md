---
# project-os-q8fy
title: Playwright e2e harness for autonomous Winamp bug iteration
status: todo
type: epic
priority: high
created_at: 2026-04-20T16:57:14Z
updated_at: 2026-04-20T16:57:14Z
---

Stand up an autonomous edit-test loop so agents can reproduce and verify fixes for the mobile Webamp drag bug (and future UI bugs) without a human in the loop.

## Background

Target bug: on mobile (Pixel 7 viewport), pressing the Webamp tray icon (bottom-right of taskbar) spawns the Webamp player. Dragging the player up/down/left/right should move only the player — but currently other fixed-positioned elements (taskbar, Clippy) also shift. The player itself should move freely.

The prior viewport-anchoring attempt was reverted (see commits `0f01c04`, `4d4bb0a`, `3ac8136`). This epic builds the test infrastructure that lets agents iterate on a fix without manual testing.

## Deliverables (child tickets)

- Nix flake: add `bun` and Playwright-compatible browser deps
- Scaffold `@playwright/test` + `playwright.config.ts` with `webServer` auto-spawning `trunk serve`
- Concrete failing repro test: Pixel 7, drag Webamp player in 4 directions, assert taskbar+Clippy positions unchanged while player position changes
- Document the autonomous loop (how an agent runs `bun run test:e2e` and iterates)

## Key DOM anchors
- Webamp tray button: `.tray-icon-btn` in `.taskbar`
- Webamp mount: `#webamp-mount` (Webamp renders its own `#main-window` etc. inside)
- Taskbar: `.taskbar`
- Clippy: `.clippy-widget`

## Non-goals
- CI integration (local agentic loop only for now)
- Cross-browser matrix (Chromium only)
- Desktop viewport coverage (mobile Pixel 7 only)
