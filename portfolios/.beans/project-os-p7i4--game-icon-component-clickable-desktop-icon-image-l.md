---
# project-os-p7i4
title: 'game icon component: clickable desktop icon (image + label)'
status: completed
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T17:58:37Z
parent: project-os-zsw7
---

Implement the GameIcon component.
- src/components/game_icon.rs: renders game.icon image + game.title label, Win95 icon styling
- Double-click or single-click opens the game window (calls open callback)
- Icons are laid out in a fixed grid; icons cannot be dragged/moved
- Placeholder asset: gray square or generic icon image

## Summary of Changes

- Created `src/components/game_icon.rs`: `GameIcon` component with props `game: Game` and `on_open: Callback<String>`; renders icon image with pixelated rendering and a label; falls back to inline SVG gray placeholder when `icon` is empty
- Added game icon CSS rules to `styles/main.css`: `.game-icon`, `.game-icon img`, `.game-icon-label`, `.game-icon:hover`
- Updated `src/components/desktop.rs`: added `on_open: Callback<String>` to `DesktopProps`, replaced placeholder spans with `<GameIcon>` components
- Updated `src/components/mod.rs`: added `pub mod game_icon`
- Updated `src/app.rs`: passes a logging no-op `on_open` callback to `<Desktop>`
- `cargo check --target wasm32-unknown-unknown` passes (6 pre-existing dead code warnings, no errors)
