---
# project-os-8npm
title: Webamp tray icon next to clock toggles single instance
status: completed
type: feature
priority: normal
created_at: 2026-04-17T23:36:36Z
updated_at: 2026-04-17T23:39:53Z
---

Add a square tray icon to the taskbar, positioned next to the clock, using `public/wm-4.png`. Clicking the icon toggles the Webamp media player:

- If no Webamp is active, spawn one.
- If a Webamp is already active, close it.

## Requirements

- [x] Render icon in the taskbar tray area adjacent to the live clock
- [x] Icon is square (equal width/height), sized to fit the taskbar height
- [x] Icon uses `public/wm-4.png`
- [x] Clicking the icon spawns Webamp when none is active
- [x] Clicking the icon closes the active Webamp when one is running
- [x] Only a single Webamp instance is allowed at a time

## Notes

Previous attempts at this feature (rolled back from commits `254e23a`..`d8b8c08`) broke Webamp interactivity — the player chrome became non-interactable. When re-implementing, verify the rendered Webamp remains fully interactive (transport controls, playlist, drag, volume slider, etc.) after mounting from the tray button.

## Summary of Changes

- `src/app.rs`: Added `webamp_active: UseStateHandle<bool>` and `on_webamp_toggle` callback. Conditioned `<Webamp>` render on `*webamp_active`. Passed `webamp_tray` (Some(active) when config has tracks, None otherwise) and `on_webamp_toggle` to `<Taskbar>`.
- `src/components/taskbar.rs`: Added `webamp_tray: Option<bool>` and `on_webamp_toggle: Callback<()>` props. Grouped tray icon + clock into a `.taskbar-tray` div. Tray icon renders only when `webamp_tray` is `Some`; button gets `active` class when value is `true`.
- `styles/main.css`: Added `.taskbar-tray`, `.tray-icon-btn`, `.tray-icon-btn.active`, and `.tray-icon-img` styles. Moved the inset border from `.taskbar-clock` to `.taskbar-tray` so it wraps both icon and clock together.

Interactivity note: the `<Webamp>` component renders as a bare `<div id="webamp-mount">` with no additional wrapper or overlay — identical to the previous unconditional render. Webamp injects its own chrome directly into that node. No pointer-event-intercepting divs were introduced, which was the root cause of the prior failures.
