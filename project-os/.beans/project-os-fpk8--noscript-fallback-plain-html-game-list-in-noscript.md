---
# project-os-fpk8
title: 'noscript fallback: plain HTML game list in <noscript> block'
status: todo
type: task
priority: low
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Add a <noscript> fallback in index.html so the page is useful without JS/WASM.
- <noscript> block in index.html contains a manually-maintained plain HTML list of games
- Each game entry: h2 title, p description, tech list, Launch/Download anchor link
- Basic CSS (inline or in noscript-scoped block) for readable layout without win95.css dependency
- Note: must be kept in sync with games.toml manually when games are added
- Add a TODO comment near the noscript block as a reminder
