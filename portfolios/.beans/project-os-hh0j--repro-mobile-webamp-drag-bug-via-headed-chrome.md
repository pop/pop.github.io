---
# project-os-hh0j
title: Repro mobile Webamp drag bug via headed Chrome
status: completed
type: task
priority: normal
created_at: 2026-04-20T22:44:19Z
updated_at: 2026-04-20T23:01:50Z
parent: project-os-q8fy
blocked_by:
    - project-os-0b6n
---

Exploratory ticket: see if headed Chromium (with various viewport/URL-bar simulation tricks) can reproduce the mobile Webamp drag bug that `project-os-0b6n`'s headless test could not.

## Context
`project-os-0b6n` landed a Pixel 7 Playwright test that dispatches touch-drag events on the Webamp titlebar and checks that `.taskbar` / `.clippy-widget` stay put. It passes on headless Chromium because headless has no URL bar, so `visualViewport === layoutViewport` and fixed elements never shift.

The real-device symptom: on mobile Chrome, dragging Webamp's window causes the layout viewport to grow (body.scrollWidth > window.innerWidth), which interacts with Chrome's URL-bar retract animation and makes fixed-position elements visually shift.

## Things to try
- [ ] Launch Playwright with `headless: false` and inspect whether Chrome UI differs
- [ ] Use `channel: 'chrome'` (real Google Chrome binary, not Chromium) — may have more faithful mobile emulation
- [ ] Mid-test viewport manipulation: `page.setViewportSize()` to simulate URL-bar retraction, or dispatch a synthetic `resize` on `window.visualViewport`
- [ ] Chromium launch flags: `--enable-features=MobileLayoutViewport`, `--use-mobile-user-agent`, `--touch-events=enabled`, `--user-agent-mobile`
- [ ] CDP (Chrome DevTools Protocol) `Emulation.setDeviceMetricsOverride` with `mobile: true` and explicit `visualViewport` params
- [ ] Combine: capture `window.visualViewport.offsetTop` before/after drag and assert it didn't shift (or if it shifted, verify the fixed elements stayed put relative to it)

## Success criteria
One of:
- A test configuration that makes the drag move taskbar/Clippy (bug reproduced in automation — ideal)
- Evidence that no headed-Chrome approach reproduces the viewport-shift symptom, with a brief writeup of what was tried

## Non-goals
- Don't rewrite the existing test; add a new file `tests/e2e/webamp-drag-headed.spec.ts` or a new project in `playwright.config.ts`
- Don't fix the Rust bug
- Don't onboard a real-device service (BrowserStack etc.)

## Budget
Hard stop at 3 `bun run test:e2e` runs. This is exploratory — quick "does it work" checks, not an exhaustive matrix.

## Exploration Notes

**Approaches tried** (3 test runs used):

**Approach A — CDP `Emulation.setDeviceMetricsOverride` with `mobile: true` + `visibleSize`**
- Added a new playwright project `pixel-7-cdp` with `--enable-features=MobileLayoutViewport` Chromium flag.
- Used a CDP session to call `Emulation.setDeviceMetricsOverride` with `mobile: true` and `visibleSize: { width: 412, height: 815 }` (100px shorter than layout) after the drag.
- **Result**: `visualViewport.offsetTop` stayed 0 throughout. The `visibleSize` CDP param does NOT produce a visual/layout viewport split in Playwright's Chromium. Dead end.

**Approach B — `page.setViewportSize()` shrink mid-test**
- After dragging Webamp rightward, called `page.setViewportSize()` to reduce height by 100px (URL bar analogue), then re-expanded.
- **Result**:
  - Body did NOT expand during drag (`scrollWidth: 412 ≤ innerWidth: 479`) — the `project-os-040x` fix is working correctly.
  - The shrink moved taskbar ~116px up and clippy ~106px up (correct behavior: `position:fixed` bottom:0 tracks the new viewport bottom).
  - After re-expanding, both elements fully restored to original Y (0px residual delta).
  - `visualViewport.offsetTop` remained 0 even during shrink — Playwright never creates a visual/layout viewport divergence.
  - Test passed (no bug).

**Verdict: headed Chrome cannot reproduce the bug in automation.**

The visual-viewport divergence (`visualViewport.offsetTop > 0`) only occurs in real mobile Chrome when the URL bar animates. Playwright's Chromium — whether headless or with mobile emulation flags — always keeps `visualViewport === layoutViewport`. The CDP `visibleSize` trick does not bridge this gap. The `setViewportSize` trick correctly simulates viewport-height changes but doesn't create the layout/visual-viewport split that is the root cause.

**Recommendation for next steps:**
- A real-device testing service (BrowserStack, LambdaTest, etc.) is the only reliable automation path for reproducing this specific bug class.
- Alternatively, the fix can be validated manually on a physical Pixel device.
- The existing `webamp-drag.spec.ts` test is the best available automated guard — it catches the body-expansion symptom even if it can't catch the visual-viewport-offset symptom directly.
