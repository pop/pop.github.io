---
# project-os-uwrj
title: Scaffold @playwright/test with trunk serve webServer
status: completed
type: task
priority: high
created_at: 2026-04-20T16:57:39Z
updated_at: 2026-04-20T17:46:09Z
parent: project-os-q8fy
blocked_by:
    - project-os-er6d
---

Set up the Playwright test runner, config, and scripts so `bun run test:e2e` boots `trunk serve`, runs tests against it in a Pixel 7 Chromium, and tears down cleanly.

## Context
Epic: `project-os-q8fy`. Depends on flake deps from `project-os-er6d`.

Target: a self-contained agent loop — agent edits Rust source, runs `bun run test:e2e`, and gets pass/fail + traces without any manual browser work.

## Todo
- [x] `bun init` a minimal `package.json` (no app, just test deps)
- [x] `bun add -d @playwright/test`
- [x] Create `playwright.config.ts` at repo root:
  - `testDir: 'tests/e2e'`
  - `projects: [{ name: 'pixel-7', use: { ...devices['Pixel 7'] } }]`
  - `webServer: { command: 'trunk serve --port 8080', url: 'http://localhost:8080', reuseExistingServer: !process.env.CI, timeout: 120_000 }`
  - `use.baseURL: 'http://localhost:8080'`
  - `reporter: [['list'], ['html', { open: 'never' }]]` so traces are inspectable
  - `use.trace: 'retain-on-failure'`, `screenshot: 'only-on-failure'`
- [x] Create `tests/e2e/` with a trivial smoke test `smoke.spec.ts` that loads `/` and asserts the `.taskbar` is visible (just to prove the pipeline works)
- [x] Add `package.json` script: `"test:e2e": "playwright test"`, `"test:e2e:ui": "playwright test --ui"`
- [x] `.gitignore`: `node_modules/`, `test-results/`, `playwright-report/`, `.playwright/`
- [x] Commit `package.json`, `bun.lockb`, `playwright.config.ts`, `tests/e2e/smoke.spec.ts`, updated `.gitignore` referencing this ticket

## Acceptance
`bun run test:e2e` from a fresh `nix develop` + `bun install` passes the smoke test, spawning and tearing down trunk automatically.

## Non-goals
- Don't write the Webamp drag repro here — that's the next ticket
- Don't configure CI

## Summary of Changes
- Updated `flake.nix` to add `bun`, `playwright-driver.browsers` to `buildInputs` and export `PLAYWRIGHT_BROWSERS_PATH`, `PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1`, `PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true` via `shellHook`
- Created `package.json` with `test:e2e` and `test:e2e:ui` scripts, `@playwright/test@1.58.2` dev dep (pinned to match nix chromium 1208)
- Created `playwright.config.ts` with Pixel 7 project, trunk webServer on port 8080, trace/screenshot on failure, html reporter
- Created `tests/e2e/smoke.spec.ts` asserting `.taskbar` visible on `/`
- Updated `.gitignore` with `node_modules/`, `test-results/`, `playwright-report/`, `.playwright/`
- Smoke test passes: 1 passed (5.9s)
