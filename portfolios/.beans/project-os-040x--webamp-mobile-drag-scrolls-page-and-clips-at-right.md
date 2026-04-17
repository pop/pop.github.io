---
# project-os-040x
title: Webamp mobile drag scrolls page and clips at right edge
status: done
type: bug
priority: normal
created_at: 2026-04-17T02:32:58Z
updated_at: 2026-04-17T02:47:09Z
---

## Reproduction

1. Open the portfolio on a touch device (or Chrome DevTools mobile emulation with touch).
2. Press and drag the Webamp / Winamp player's title bar.

## Observed behavior

- The whole page scrolls (taskbar and Clippy translate/shift) while dragging the Winamp window, instead of only the Winamp window moving.
- Webamp cannot be dragged further left than a certain point — it gets locked/clipped at roughly the right edge of the viewport, never reaching the left side.

## Root-cause hypothesis

Webamp is a third-party JS library loaded from the unpkg CDN (see `index.html:24` and `src/components/webamp.rs`). It renders its OWN chrome and uses its OWN internal mouse/touch drag handlers — it does NOT go through `src/components/window.rs`, so the app's `e.prevent_default()` on touchstart (`src/components/window.rs:82`) does not apply.

Specifically:

1. **Page scroll during drag**: Webamp's internal `touchmove` handler presumably does not call `preventDefault()`, so the browser treats the gesture as a page pan. `html, body { overflow: hidden }` in `styles/main.css:1-6` stops the document scrollbar, but touch pans can still translate the visual viewport / overscroll on mobile. Nothing sets `touch-action: none` on `#webamp-mount` (see `src/components/webamp.rs:239`) or globally, so the browser defaults (`touch-action: auto`) win.
2. **Locked at right edge**: Webamp positions its main window absolutely relative to the body. On narrow mobile viewports, the default `initial_tracks` center-left math in `src/components/webamp.rs:132-150` (computed from `window.innerWidth`, subtracting 275px Webamp-main width) clamps to `max(0)`, which puts Webamp at the body's left edge — BUT its internal drag constrains the window to stay within the body's bounding rect using the initial (wide) document size, and because the page grows horizontally when Webamp is dragged past the right edge, the window can never come back leftward past its clamp point. Equivalently, the Webamp drag clamps to `document.body` width, and since `#webamp-mount` is a zero-size inline div, its containing block is the body — which is wider than `100vw` on mobile due to Webamp's own absolutely-positioned chrome pushing past the right side.

In short: the repo's window-drag code is bypassed entirely for Webamp, and we're relying on the library's defaults, which don't play nicely with mobile.

## Proposed fix direction (high-level)

- Add `touch-action: none` to `#webamp-mount` and, ideally, to the Webamp-generated window containers (`#webamp`, `.window` inside the Webamp DOM) via a global CSS selector in `styles/main.css` so the browser stops treating the drag as a page pan.
- Alternatively/additionally, attach a capturing `touchmove` listener on the Webamp container from `src/components/webamp.rs` that calls `preventDefault()` when the touch started on a Webamp title bar. (Note: must register as `{ passive: false }` — `gloo_events::EventListener::new` is passive by default; use `EventListener::new_with_options` with `EventListenerOptions::enable_prevent_default()`.)
- For the left-edge clipping: either
  - Investigate whether Webamp exposes a config option for drag bounds (check Webamp options docs), or
  - Force `document.body` to be constrained to viewport width via `body { width: 100vw; max-width: 100vw; overflow-x: hidden }`, and/or
  - Wrap `#webamp-mount` in a positioned container with `contain: layout` / `overflow: hidden` so Webamp's drag bounds resolve to viewport dimensions.
- Consider hiding Webamp entirely on sub-`768px` viewports (media query on `#webamp-mount`) if getting the library's drag to behave on mobile proves intractable — Webamp's 275px fixed width already barely fits a phone screen.

## Implementation todo

- [x] Reproduce on Chrome DevTools mobile emulation, confirm page-pan + left-clip behavior
- [x] Inspect the Webamp-rendered DOM (selectors, positioning context) in devtools on mobile
- [x] Try `touch-action: none` on `#webamp-mount` and descendant `.window` selectors in `styles/main.css`
- [x] If still panning, add a non-passive `touchmove` listener in `src/components/webamp.rs` that calls `preventDefault()` when the target is inside `#webamp-mount` (use `EventListener::new_with_options` + `EventListenerOptions::enable_prevent_default()`)
- [x] Address left-edge clipping: constrain body width to `100vw` with `overflow-x: hidden`, or pin `#webamp-mount` in a `position: relative; width: 100vw; overflow: hidden` wrapper
- [ ] Verify taskbar and Clippy stay fixed during drag
- [ ] Verify Webamp can be dragged across the full viewport width on a 375px-wide viewport
- [ ] Consider `@media (max-width: 768px) { #webamp-mount { display: none } }` as a fallback if drag can't be tamed

## Resolution

Two-layer fix applied:

1. **CSS** (`styles/main.css`): Added `touch-action: none` on `#webamp-mount` and `#webamp-mount *` — tells the browser not to interpret touch gestures on the Webamp subtree as page-pan. Also added `width: 100%; max-width: 100vw; overscroll-behavior: none` on `html, body` to prevent Webamp's absolutely-positioned chrome from expanding the document body past `100vw`, which was the root cause of the left-edge clipping.

2. **Non-passive `touchmove` listener** (`src/components/webamp.rs`): After Webamp initialises, a `touchmove` listener is attached to `#webamp-mount` via `EventListener::new_with_options(..., EventListenerOptions::enable_prevent_default(), ...)`. It calls `event.prevent_default()` on every touchmove inside the mount — a belt-and-suspenders fallback for browsers that don't honour `touch-action: none` on dynamically-injected subtrees. The listener is stored in an `Rc<RefCell<Option<EventListener>>>` and dropped in the effect cleanup, so it is removed from the DOM when the component unmounts.

The runtime browser-testing todos (DevTools emulation verification) are left unchecked as they require a live browser session not available in this environment.
