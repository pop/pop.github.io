---
# project-os-v3e7
title: 'game window content: overview panel inside window'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Implement the inner content of a game overview window.
- src/components/game_window.rs: renders inside Window shell
- Layout: game title (h2), contributors list, tech stack row (each tech: gif icon + name), description paragraph, demo media (img or video autoplay loop muted), Launch/Download button
- Launch button: window.open(launch_url, _blank) — label is Launch if launch_type==wasm, Download otherwise
- Placeholder: gray boxes for missing assets
