---
# project-os-p2th
title: 'start menu popup: About / GitHub / itch.io / Shut Down'
status: completed
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T18:06:16Z
parent: project-os-zsw7
---

Implement the Start menu popup.
- src/components/start_menu.rs: shown when Start button is clicked, hidden on click-outside or second click
- Menu items (from config start_menu): About (about_url), GitHub (github_url), itch.io (itchio_url), separator, Shut Down... (fun easter egg: alert or displays funny Win95 message)
- Links open in new tab
- Win95 popup/menu styling from win95.css
- Menu appears above taskbar, closes on any click outside

## Summary of Changes

- Created `src/components/start_menu.rs`: `StartMenuComp` with backdrop close, About/GitHub/itch.io links (open in `_blank`), and Shut Down easter egg alert
- Updated `src/components/mod.rs`: added `pub mod start_menu`
- Updated `src/app.rs`: import and render `<StartMenuComp>` above `<Taskbar>`
- Updated `src/config.rs`: added `PartialEq` to `StartMenu` derive (required by `Properties`)
- Updated `styles/main.css`: added start menu backdrop, popup, list, item, and shutdown styles
- `cargo check --target wasm32-unknown-unknown` passes with no errors
