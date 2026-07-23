---
# project-os-cua3
title: 'fix: game icons use emoji as img src causing doubled text'
status: completed
type: bug
priority: high
created_at: 2026-04-08T18:31:14Z
updated_at: 2026-04-08T18:34:29Z
parent: project-os-zsw7
---

games.toml stores icons as emoji strings (🐍, 🧱, 🦀, etc). game_icon.rs and game_window.rs pass these directly as <img src=...> which fails to load, causing the browser to display the alt text inline plus the explicit label text below — resulting in doubled text (e.g. 'Snake' appears twice, tech shows 'Ru Rust'). Fix: add an is_url() helper in both components — render as <img> when the value is a URL, as a <span> with the emoji text otherwise. Add CSS for .game-icon-emoji (36px font) and .tech-icon-emoji (14px font).

## Summary of Changes

- src/components/game_icon.rs: Added is_url() helper; replaced unconditional img with a conditional that renders a span.game-icon-emoji for emoji values and falls back to an SVG placeholder when the icon is empty.
- src/components/game_window.rs: Same is_url() helper added; tech stack icon rendering now uses span.tech-icon-emoji for emoji values instead of a broken img src.
- styles/main.css: Added .game-icon-emoji (36px, 48x48 flex container) and .tech-icon-emoji (14px) rules so emoji icons are sized correctly in both contexts.
