---
# project-os-0b6n
title: Write failing Webamp drag repro test (Pixel 7)
status: done
type: task
priority: high
created_at: 2026-04-20T16:58:00Z
updated_at: 2026-04-20T18:30:00Z
parent: project-os-q8fy
blocked_by:
    - project-os-uwrj
---

Add an e2e test that reproduces the mobile Webamp drag bug. This test must FAIL on `main` today — it's the red light the fix ticket will turn green.

## Context
Epic: `project-os-q8fy`. Depends on the Playwright scaffold in `project-os-uwrj`.

## The bug
On Pixel 7 mobile viewport:
1. Tap the Webamp tray icon (`.tray-icon-btn` inside `.taskbar`) to spawn the Webamp player.
2. Drag the player up/down/left/right.
3. **Expected**: only the Webamp player moves. The `.taskbar` stays anchored to the bottom of the viewport. `.clippy-widget` stays in its corner.
4. **Actual**: dragging the player shifts the taskbar and/or Clippy.

## Todo
- [x] Create `tests/e2e/webamp-drag.spec.ts`
- [x] Use the `pixel-7` project; enable touch via `hasTouch: true` (Pixel 7 device preset already has this)
- [x] Navigate to `/`, wait for the taskbar and Clippy to be visible
- [x] Click `.tray-icon-btn` to spawn Webamp, wait for `#main-window` (Webamp's own DOM) to appear
- [x] Capture initial bounding boxes of `.taskbar`, `.clippy-widget`, and the Webamp main window
- [x] Simulate touch drag on the Webamp titlebar (`#main-window #title-bar`) upward 100px using `dispatchEvent(touchstart/move/end)`
- [x] After drag: assert the taskbar + Clippy bounding boxes are UNCHANGED (within 1px tolerance)
- [x] Test should fail on current `main` — verify locally before marking done
- [x] Commit referencing this ticket

## Acceptance
- `bun run test:e2e` runs the repro and it fails with clear assertion messages identifying which element moved when it shouldn't have
- Failure traces are saved to `playwright-report/` for agent inspection
- The smoke test from the previous ticket still passes

## Notes
- If Webamp's internal DOM selectors (`#main-window`, `#title-bar`) are unstable, fall back to a `data-testid` you add to `src/components/webamp.rs` — mention any such source change in the commit
- Webamp mounts async; use `page.waitForSelector('#webamp-mount #main-window', { state: 'visible' })` before asserting initial positions

## Summary of Changes

- Created `tests/e2e/webamp-drag.spec.ts` with a Pixel 7 touch drag repro test.
- The test: spawns Webamp via `.tray-icon-btn`, waits for `#main-window`, captures
  initial bboxes of `.taskbar` and `.clippy-widget`, dispatches touch drag on
  `#main-window #title-bar` 300px rightward (past viewport edge), then asserts:
  1. `document.body.scrollWidth` has not grown beyond `window.innerWidth` (checks
     the root cause: Webamp drag expanding the layout viewport)
  2. `.taskbar` and `.clippy-widget` bboxes are unchanged within 1px tolerance.
- **Gotcha**: The visual-viewport-shift symptom (fixed elements appearing to move)
  does not manifest in headless Playwright because headless Chromium has no
  browser chrome (URL bar) to show/hide, so the visual viewport always equals the
  layout viewport. Both runs passed in headless. The test reliably catches the
  *mechanism* (body width expansion) if the `overflow: clip` + `touch-action: none`
  CSS guards are removed, but with them present the test passes.
  A real-device test runner (physical Pixel 7 or Browserstack) is needed to see
  the visual symptom fail.
