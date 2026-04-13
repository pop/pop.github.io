---
# project-os-d25a
title: 'taskbar: open-window buttons + live clock + Start button'
status: completed
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T18:04:32Z
parent: project-os-zsw7
---

Implement the Win95 taskbar pinned to the bottom.
- src/components/taskbar.rs: fixed bottom bar using win95.css panel styles
- Start button on far left (triggers start menu show/hide)
- Center section: one button per open window (shows game title); clicking brings window to front (on_focus callback)
- Clock on far right: formatted HH:MM, updates every 1s via gloo_timers Interval
- Integrate with WindowManager state in App

## Summary of Changes

- Created `src/components/taskbar.rs`: Win95-styled fixed bottom bar with Start button (toggles start_menu_open state), per-open-window buttons that call on_focus, and a live clock (HH:MM AM/PM) updated every 1s via gloo_timers Interval.
- Updated `src/components/mod.rs`: added `pub mod taskbar`.
- Updated `src/app.rs`: added `start_menu_open` state, `on_start_click` toggle callback, `on_taskbar_focus` callback, and rendered `<Taskbar>` at root level.
- Updated `styles/main.css`: added flexbox layout rules for .taskbar, .start-button, .taskbar-windows, .taskbar-window-btn, and .taskbar-clock.
- `cargo check --target wasm32-unknown-unknown` passes (3 pre-existing dead_code warnings, no errors).
