---
# project-os-fpk8
title: 'noscript fallback: plain HTML game list in <noscript> block'
status: completed
type: task
priority: low
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T18:13:26Z
parent: project-os-zsw7
---

Add a <noscript> fallback in index.html so the page is useful without JS/WASM.
- <noscript> block in index.html contains a manually-maintained plain HTML list of games
- Each game entry: h2 title, p description, tech list, Launch/Download anchor link
- Basic CSS (inline or in noscript-scoped block) for readable layout without win95.css dependency
- Note: must be kept in sync with games.toml manually when games are added
- Add a TODO comment near the noscript block as a reminder

## Summary of Changes

- Added a noscript block to index.html inside body, before Trunk's injection point
- Block includes inline CSS with .ns-container, .ns-game, .ns-tech, .ns-launch classes — no win95.css dependency
- Populated with both games from games.toml: Snake (Rust, WebAssembly) and Tetris (Rust, WebAssembly, Canvas)
- Each game entry has: icon + title in h2, description in p, tech stack as ul, and a Launch anchor pointing to the game's launch_url
- TODO comment added reminding future editors to keep this in sync with games.toml manually
- cargo check --target wasm32-unknown-unknown passes (no Rust changes made)
