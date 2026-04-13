---
# project-os-v3e7
title: 'game window content: overview panel inside window'
status: completed
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T18:02:45Z
parent: project-os-zsw7
---

Implement the inner content of a game overview window.
- src/components/game_window.rs: renders inside Window shell
- Layout: game title (h2), contributors list, tech stack row (each tech: gif icon + name), description paragraph, demo media (img or video autoplay loop muted), Launch/Download button
- Launch button: window.open(launch_url, _blank) — label is Launch if launch_type==wasm, Download otherwise
- Placeholder: gray boxes for missing assets

## Summary of Changes

- Created `src/components/game_window.rs`: `GameWindow` component with game title, contributors, tech stack (icons + names), description, demo media (video/image/placeholder), and launch/download button.
- Updated `src/components/mod.rs`: added `pub mod game_window`.
- Updated `src/app.rs`: window rendering now looks up the `Game` by `game_id`, passes `game.title` to the `Window` title bar, and renders `<GameWindow game={game} />` as window content.
- Updated `styles/main.css`: added CSS for `.game-window-content`, `.contributors`, `.tech-stack`, `.tech-item`, `.description`, `.game-demo`, `.game-demo-placeholder`, and `.launch-row`.
- `cargo check --target wasm32-unknown-unknown` passes (3 pre-existing dead code warnings, no new errors).
