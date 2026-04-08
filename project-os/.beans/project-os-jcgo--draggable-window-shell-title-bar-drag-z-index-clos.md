---
# project-os-jcgo
title: 'draggable window shell: title bar drag, z-index, close button'
status: completed
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T18:00:57Z
parent: project-os-zsw7
---

Generic Win95-style window with drag and z-index management.
- src/components/window.rs: takes title, z_index, pos, on_close, on_focus, children props
- Title bar: game title left, X button right (Win95 style from win95.css)
- Dragging: onmousedown on title bar records offset; document-level onmousemove updates pos; onmouseup clears drag. Use gloo-events EventListener on document.
- Touch: ontouchstart/ontouchmove/ontouchend equivalents
- Clicking anywhere on window calls on_focus (increments z_counter, assigns to this window)
- Position via inline style: position:absolute, left, top, z-index
- Desktop: position:relative, overflow:hidden (windows clipped to desktop area)

## Summary of Changes

- Created `src/components/window.rs`: Win95-style window component with title-bar mouse/touch drag via document-level `gloo_events::EventListener`, z-index focus-on-click, and close button
- Updated `src/app.rs`: holds `WindowManager` in `use_state`, wires `on_open`/`on_close`/`on_focus`/`on_move` callbacks for each open window
- Updated `src/components/mod.rs`: added `pub mod window`
- Updated `Cargo.toml`: added `MouseEvent`, `TouchEvent`, `Touch`, `TouchList`, `EventTarget` to web-sys features
- Added window CSS to `styles/main.css`: grab cursor on title bar, position:absolute container
- `cargo check --target wasm32-unknown-unknown` passes (3 pre-existing dead_code warnings only)
