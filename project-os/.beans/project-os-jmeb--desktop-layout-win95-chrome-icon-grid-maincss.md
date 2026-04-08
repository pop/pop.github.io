---
# project-os-jmeb
title: 'desktop layout: Win95 chrome, icon grid, main.css'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Wire up Win95 visual theme and desktop layout.
- index.html: link win95.css CDN (https://alexbsoft.github.io/win95.css/win95.css), link styles/main.css via data-trunk, empty body for Yew, noscript block (see noscript ticket)
- styles/main.css: full-viewport desktop div with Win95 teal background, icon grid layout (top-left, wraps), taskbar pinned to bottom, Clippy pinned bottom-right above taskbar
- src/components/desktop.rs: Desktop component renders icon grid from config games
- Verify: visual output looks like a Win95 desktop
