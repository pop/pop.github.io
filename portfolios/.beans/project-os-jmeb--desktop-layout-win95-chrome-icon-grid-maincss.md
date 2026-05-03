---
# project-os-jmeb
title: 'desktop layout: Win95 chrome, icon grid, main.css'
status: completed
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T17:57:20Z
parent: project-os-zsw7
---

Wire up Win95 visual theme and desktop layout.
- index.html: link win95.css CDN (https://alexbsoft.github.io/win95.css/win95.css), link styles/main.css via data-trunk, empty body for Yew, noscript block (see noscript ticket)
- styles/main.css: full-viewport desktop div with Win95 teal background, icon grid layout (top-left, wraps), taskbar pinned to bottom, Clippy pinned bottom-right above taskbar
- src/components/desktop.rs: Desktop component renders icon grid from config games
- Verify: visual output looks like a Win95 desktop

## Summary of Changes

- Updated `index.html` with win95.css CDN link and trunk CSS link for `styles/main.css`
- Created `styles/main.css` with `#desktop`, `.icon-grid`, `.taskbar`, and `.clippy-widget` rules
- Created `src/components/mod.rs` declaring `pub mod desktop`
- Created `src/components/desktop.rs` with `Desktop` component taking `Vec<Game>` prop and rendering icon-grid placeholders
- Updated `src/app.rs` to call `load_config()` and render `<Desktop games={config.games} />`
- Added `mod components` to `src/main.rs`
- Added `#[derive(PartialEq)]` to `Game` and `Tech` in `src/config.rs` (required by Yew `Properties` derive)
- `cargo check --target wasm32-unknown-unknown` passes (6 dead_code warnings only, expected for scaffolded fields)
