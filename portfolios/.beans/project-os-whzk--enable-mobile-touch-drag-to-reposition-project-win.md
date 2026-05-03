---
# project-os-whzk
title: Enable mobile touch drag to reposition project Windows
status: completed
type: bug
priority: high
created_at: 2026-04-23T20:51:45Z
updated_at: 2026-04-23T20:56:01Z
---

## Problem

On mobile (viewport <= 768px), pressing and dragging a project Window's title bar does not move the window. The touch handler runs and `WindowManager.pos` updates, but the window stays pinned in place.

Desktop drag works. Webamp mobile drag works (project-os-040x). Clippy mobile drag works. Only the regular project Windows are stuck.

## Root cause

Commit `4250fb1` (project-os-rcgj) wired a non-passive `touchstart` listener on `.window .title-bar` via `gloo_events::EventListener::new_with_options` in `src/components/window.rs:82-140`. It calls `preventDefault()`, tracks the finger via document-level `touchmove`, and emits `on_move`. `app.rs` updates `WindowManager.pos`, which re-renders the inline `style="left:Xpx; top:Ypx"`.

The mobile CSS in `styles/main.css:333-343` defeats this:

- `position: fixed !important;`
- `left: 18vw !important;`
- `top: 8px !important;`
- `transform: translateX(var(--jitter-x)) translateY(var(--jitter-y));`

The `!important` on `left`/`top` overrides the inline style, and the jitter transform reads CSS vars set once at mount (`window.rs:31-32`), never updated by drag. Net effect: `on_move` fires, state updates, DOM updates, but CSS pins the box.

This was intentional in prior tickets (`project-os-a1el`, `project-os-p2fc`) when mobile windows were designed static. That constraint is now being lifted.

## Proposed approach

Make mobile `.window` positioning driven by the same inline `left`/`top` that desktop uses.

1. `styles/main.css` (`@media (max-width: 768px) .window`):
   - Remove `left: 18vw !important;`
   - Remove `top: 8px !important;`
   - Remove the `transform: translate...` jitter line
   - Keep `position: fixed !important;` (mobile desktop doesnt scroll, and `client_x/y` are viewport coords)
   - Keep `width: 80vw !important;` and min-width/min-height/z-index rules

2. `src/components/window.rs`:
   - Remove `--jitter-x` / `--jitter-y` CSS custom properties from inline style string
   - Remove any `use_state` for jitter if unused after CSS change

3. `src/state.rs` `WindowManager::new` initial pos:
   - Current `(50 + i*30, 50 + i*30)` may land mostly off-screen at 80vw on narrow viewport. Leave as-is (first drag snaps into view) unless testing shows title bar is unreachable.

## Tasks

- [x] Remove `left: 18vw !important;` and `top: 8px !important;` from mobile `.window` rule in `styles/main.css`
- [x] Remove the jitter `transform` line from the same rule
- [x] Remove `--jitter-x` / `--jitter-y` setup from `src/components/window.rs` if unused
- [x] `cargo check --target wasm32-unknown-unknown`
- [x] `cargo clippy --target wasm32-unknown-unknown`
- [ ] `trunk serve`, verify desktop drag still works
- [ ] Verify mobile drag in Chrome DevTools (Pixel 7, iPhone SE viewports)
- [ ] Verify page does not scroll while dragging title bar
- [ ] Verify close button, focus-on-click, and window-body scroll still work
- [ ] Verify Clippy AI modal override (`styles/main.css:352-357`) still centers
- [ ] Verify Start menu override (`styles/main.css:360-366`) still anchors bottom-left

## Testing notes

Repro: load dev server in Chrome, toggle device toolbar, pick Pixel 7, click a project icon, press-and-drag the title bar. Expected: window follows finger. Actual: window does not move.

## Related

- `project-os-rcgj` (4250fb1) — added the touch listener this ticket makes useful
- `project-os-a1el` / `project-os-p2fc` — introduced the static mobile layout this partially reverses
- `project-os-040x` — sibling fix for Webamp mobile drag

## Summary of Changes

- `styles/main.css`: Removed `left: 18vw !important;`, `top: 8px !important;`, and `transform: translateX(var(--jitter-x, 0px)) translateY(var(--jitter-y, 0px));` from the `@media (max-width: 768px) .window` rule. Kept `position: fixed !important;`, `width: 80vw !important;`, `min-width: unset !important;`, `min-height: unset !important;`, and `z-index: 500`. Mobile windows now use the same inline `left`/`top` style updated by the drag handler.
- `src/components/window.rs`: Removed the two `use_state` jitter variables (`jitter_x`, `jitter_y`) and their `--jitter-x`/`--jitter-y` CSS custom property injection from the inline style format string. Also removed an inline reference to `js_sys::Math::random()` which was only used by those variables. `cargo fmt` reordered imports alphabetically.
- All non-browser tasks pass (`cargo check`, `cargo clippy -D warnings`, `cargo fmt`). Browser testing items left unchecked (cannot test in browser from agent environment).
