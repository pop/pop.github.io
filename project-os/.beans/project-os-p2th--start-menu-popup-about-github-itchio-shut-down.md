---
# project-os-p2th
title: 'start menu popup: About / GitHub / itch.io / Shut Down'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Implement the Start menu popup.
- src/components/start_menu.rs: shown when Start button is clicked, hidden on click-outside or second click
- Menu items (from config start_menu): About (about_url), GitHub (github_url), itch.io (itchio_url), separator, Shut Down... (fun easter egg: alert or displays funny Win95 message)
- Links open in new tab
- Win95 popup/menu styling from win95.css
- Menu appears above taskbar, closes on any click outside
