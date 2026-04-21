---
# project-os-s0wm
title: Mobile Webamp drag shifts taskbar/Clippy in Firefox (mobile preview)
status: todo
type: bug
priority: critical
created_at: 2026-04-21T04:33:23Z
updated_at: 2026-04-21T04:33:23Z
---

When the Webamp player is dragged on a mobile viewport in **Firefox** (including Desktop Firefox's Responsive Design Mode), the taskbar and/or Clippy widget visually shift relative to the viewport instead of staying anchored as `position: fixed` elements. Dragging should move only the Webamp window.

## Reproduction

**Reproduces:** Desktop Firefox → DevTools → Responsive Design Mode → any mobile device preset (Pixel 7, iPhone 13, etc.)

**Does NOT reproduce:**
- Desktop/mobile Chrome (verified by user)
- Playwright Chromium with Pixel 7 device emulation (tested `project-os-0b6n`)
- Headed Chromium with CDP `Emulation.setDeviceMetricsOverride` + `--enable-features=MobileLayoutViewport` (tested `project-os-hh0j`)
- Headed Chromium with mid-test `page.setViewportSize()` shrink (tested `project-os-hh0j`)
- **Playwright Firefox** with `hasTouch: true`, Pixel 7 viewport, mobile userAgent (tested this session)

## Steps to reproduce

1. Open the deployed site (games.elijah.run) in Desktop Firefox
2. Open DevTools → toggle Responsive Design Mode (Ctrl+Shift+M)
3. Select a mobile device preset (e.g., Pixel 7)
4. Reload the page
5. Tap the Winamp tray icon in the bottom-right of the taskbar to spawn the Webamp player
6. Press-and-drag the Webamp title bar up/down/left/right
7. Observe: the taskbar and/or Clippy widget visually shift during the drag. Expected: only the Webamp window moves.

## Likely cause

Probably a **Webamp internal bug** — not our code. The prior investigations found:

- Our `position: fixed` anchoring is correct for the layout viewport
- `project-os-040x` already clamps `body.scrollWidth ≤ window.innerWidth` during drag
- `project-os-lhw3` attempt to re-anchor taskbar/Clippy to `window.visualViewport` was reverted (didn't help)
- `project-os-ym2f` attempt at Webamp `renderInto()` with relative-positioned container was scrapped (not in Webamp 2.2.0 public API)

The symptom appears only under Firefox's mobile-emulation model, which handles the layout/visual viewport split differently than Chrome's. Webamp's drag implementation likely produces a transient DOM or style state (e.g., temporary body width change, transform on body, absolute positioning outside the viewport) that Firefox's viewport resolver reacts to but Chrome's does not.

## Why automation can't currently catch this

| Environment | Result |
|---|---|
| Playwright headless Chromium (Pixel 7) | Bug not present |
| Playwright Firefox (mobile viewport + touch) | Bug not present |
| Playwright headed Chromium + CDP mobile overlay | Bug not present |
| Firefox Responsive Design Mode | Bug reproduces |

Playwright's Firefox is a patched Gecko build without DevTools RDM; it doesn't replicate RDM's touch event synthesis or meta-viewport handling. Catching this in automation would require either:
- A real-device service (BrowserStack, LambdaTest) — cost + external dep
- Launching actual Firefox (not playwright-firefox) with `about:config` `devtools.responsive.*` prefs set and some way to toggle RDM programmatically — unproven
- Filing upstream and waiting for a fix in Webamp

## Key DOM anchors (for when a workaround is written)

- Webamp tray button: `.taskbar .tray-icon-btn`
- Webamp mount container: `#webamp-mount` (note: **Webamp injects `#main-window` into `<body>` directly**, not inside the mount element)
- Webamp main window: `#main-window`
- Webamp title bar (drag handle): `#main-window #title-bar`
- Taskbar: `.taskbar`
- Clippy: `.clippy-widget`

## Existing guardrails (already on source)

- `project-os-040x` — clamps Webamp drag so `body.scrollWidth` cannot exceed viewport width
- `project-os-lzqc` — `overflow: clip` on root container
- `project-os-8npm` — tray icon for respawning Webamp
- `tests/e2e/webamp-drag.spec.ts` — headless Pixel 7 Playwright test that guards against the *root-cause class* of bug (body expansion + fixed-element drift). Passes on main, would catch a regression of the scrollWidth guard.

## Related open tickets

- `project-os-4umw` (todo) — clamp Webamp window positions via MutationObserver (alternative containment approach)
- `project-os-00bs` (todo) — clip Webamp inside a fixed-viewport wrapper (alternative containment approach)

Either of these, if implemented, may coincidentally mitigate the Firefox symptom even though they target the Chrome-observable root cause.

## Possible workaround strategies (for a future session)

1. **Pin Webamp inside a transform-scoped container** — wrap `#webamp-mount` in a `transform: translate3d(0,0,0)` or `contain: layout` element so any Webamp-internal layout changes don't propagate to the root layout viewport.
2. **Intercept Webamp's drag with our own constrained drag** — replace Webamp's title-bar `mousedown` handler at the DOM level to use our own position state, keeping the player inside a subpixel rect that Firefox won't re-resolve from.
3. **Detect Firefox mobile-preview and disable Webamp dragging entirely** — fallback: on Firefox + mobile viewport, make Webamp non-draggable (position fixed at a sane default). Ugly but guaranteed.
4. **File upstream at https://github.com/captbaritone/webamp/issues** with a minimal repro (plain HTML page loading Webamp, dragged in Firefox RDM) — upstream fix is the cleanest long-term answer.

## When picking this up

Start by filing the upstream Webamp issue — it provides leverage even if we ship a local workaround. Then pick one of the containment strategies above (transform-scope or MutationObserver via `project-os-4umw`) and verify manually in Firefox RDM.

The e2e harness (`bun run test:e2e` from `portfolios/`) can be used to verify we don't regress the Chrome-observable guards, but manual Firefox RDM verification is the acceptance test.
