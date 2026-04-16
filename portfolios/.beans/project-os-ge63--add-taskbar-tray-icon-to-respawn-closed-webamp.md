---
# project-os-ge63
title: Add taskbar tray icon to respawn closed Webamp
status: todo
type: feature
created_at: 2026-04-16T23:33:53Z
updated_at: 2026-04-16T23:33:53Z
---

## Problem

The Webamp player has a close (X) button on its window chrome. Clicking it dispatches Webamp's CLOSE_WINAMP redux action, which makes the player vanish until the page is reloaded. There's currently no way for the user to bring the player back without a hard refresh.

## Goal

Add a small system-tray-style icon to the right edge of the taskbar — between `taskbar-windows` and `taskbar-clock` — that, when clicked, restores the Webamp window. Mirror the Win95/XP system tray convention: a thin icon strip just to the left of the clock.

## Design notes

- **Icon**: pick a Winamp / music-note style 16px icon and drop it in `public/`. Reuse the existing icon convention from `portfolios.toml` (`taskbar.start_icon`, `clippy.icon`) so the path is configurable, e.g. `taskbar.tray_webamp_icon`.
- **Visibility**: the tray slot should always be visible when the `[webamp]` section is configured (so users discover it before they even close the player). Don't conditionally hide it based on whether the player is currently open — that's a discoverability trap.
- **Click behavior**: call Webamp's public `reopen()` method on the existing instance (see `webampLazy.tsx` in webamp source — it's the documented complement to `close()`). This preserves the current track, playback position, skin, and window layout.

## Implementation sketch

1. **`src/components/webamp.rs`**
   - Add a new `#[wasm_bindgen(method, js_name = reopen)] fn reopen(this: &JsWebamp);` to the extern block.
   - Expose a way for the parent (App) to invoke `reopen()` on the live instance. Two options:
     a. Lift the `Rc<RefCell<Option<JsWebamp>>>` instance handle out of `use_effect_with` via a context or a callback prop (`on_ready: Callback<WebampHandle>`).
     b. Stash the instance on a window global (`window.__webamp`) from the init closure, and have the taskbar's onclick reach it via wasm-bindgen. Less clean but smaller diff.
   - Recommended: (a). Add a `WebampHandle` newtype around the Rc so it's PartialEq-able and cloneable, emit it via `use_effect_with` once the instance is constructed.

2. **`src/components/taskbar.rs`**
   - Add a new prop `on_webamp_reopen: Option<Callback<()>>` and `tray_webamp_icon: Option<String>`.
   - Render an `<img>`/`<button>` inside a new `<div class="taskbar-tray">` placed between `.taskbar-windows` and `.taskbar-clock` only when both the icon and callback are `Some`.

3. **`src/app.rs`**
   - Hold the `WebampHandle` in a `use_state`, set it from the `<Webamp on_ready=...>` callback, and pass a `Callback::from(move |_| if let Some(h) = handle.as_ref() { h.reopen(); })` into Taskbar.

4. **`src/config.rs`**
   - Add `tray_webamp_icon: Option<String>` to `TaskbarConfig` so the icon path is data-driven from `portfolios.toml`.

5. **`portfolios.toml`**
   - Add the new icon under `[taskbar]`, e.g. `tray_webamp_icon = "/winamp-tray.png"`.
   - Drop the actual asset under `public/` (force-add past `public/` gitignore).

6. **`styles/main.css`**
   - Add `.taskbar-tray` styles: small inset-bordered slot, vertically centered, ~22px tall, padded a few px from the clock, cursor: pointer, hover state.

## Acceptance

- [ ] Tray icon visible on the taskbar to the immediate left of the clock
- [ ] Hovering the icon shows a tooltip via `title="Open music player"`
- [ ] After clicking the X on the Webamp main window, clicking the tray icon brings the player back with the same track loaded and the same window layout
- [ ] Tray icon does NOT render if the `[webamp]` section is missing from `portfolios.toml`
- [ ] `cargo check --target wasm32-unknown-unknown` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown` passes

## Open questions

- Should the icon also reflect a 'closed' vs 'open' state visually (e.g. dimmer when player is open)? Probably not for v1 — keep it simple.
- Multi-click while the player is already open: `reopen()` is idempotent based on the Webamp source, so safe to no-op.
