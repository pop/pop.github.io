---
# project-os-rcgj
title: Enable touch drag for project Window and Clippy widget
status: in-progress
type: bug
priority: high
created_at: 2026-04-21T05:00:25Z
updated_at: 2026-04-21T05:04:22Z
---

Mobile drag broken for project Windows and Clippy. Details in body.

## Problem

On mobile, project Windows (spawned by clicking a desktop game icon) and the Clippy widget cannot be dragged. On desktop, mouse drag works fine. Different codepath from Webamp (which uses Webamp's own internal drag).

## Current state

- `src/components/window.rs` — has both `onmousedown` AND `ontouchstart` wired on `.title-bar`, but user reports touch drag does not actually work on mobile. Needs diagnosis + fix.
- `src/components/clippy.rs` — Clippy icon and modal title bar each have `onmousedown` but NO touch equivalent. Need `ontouchstart` handlers added, mirroring the mouse flow.

## Likely blockers

1. Yew's `ontouchstart` prop yields a passive event listener. A passive listener cannot `preventDefault()`, so the browser may treat the gesture as a page-pan and the `touchmove` deltas are never received. Fix pattern already used in `webamp.rs`: attach via `gloo_events::EventListener::new_with_options(..., EventListenerOptions::enable_prevent_default(), ...)`.
2. Missing `touch-action: none` on the drag handle(s) in CSS. Check `styles/main.css` for `.title-bar` and the Clippy icon — add `touch-action: none` where needed so the browser does not intercept the gesture.

## Files

- `src/components/window.rs` — `.title-bar` drag (existing touchstart — make it actually work)
- `src/components/clippy.rs` — Clippy icon (`.clippy-icon`) drag; Clippy modal `.title-bar` drag
- `styles/main.css` — `touch-action: none` on the three drag handles

## Constraints

- Do NOT break desktop mouse drag.
- Do NOT break Clippy's existing `has_dragged` flag / click-vs-drag disambiguation. Touch drag must set the same flag so a tap still opens the modal but a drag does not.
- Prefer mirroring `window.rs`'s current touch pattern over inventing a new one.

## Todo

- [x] Diagnose why `window.rs` touchstart does not currently drag on mobile (likely passive listener + missing touch-action)
- [x] Fix `Window` component touch drag (project windows draggable on mobile)
- [x] Add touch drag to Clippy icon (mirrors existing `onmousedown` logic, including `has_dragged` flag)
- [x] Add touch drag to Clippy modal title bar
- [x] Add `touch-action: none` CSS where required
- [x] `cargo check --target wasm32-unknown-unknown` passes
- [x] `cargo clippy --target wasm32-unknown-unknown` passes with no new warnings
- [ ] Manual test on mobile emulation: drag a project window, drag Clippy, drag the Clippy modal
- [ ] Verify desktop mouse drag still works for all three
