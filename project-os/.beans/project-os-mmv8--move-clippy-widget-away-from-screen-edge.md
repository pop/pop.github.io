---
# project-os-mmv8
title: Move Clippy widget away from screen edge
status: completed
type: bug
priority: normal
created_at: 2026-04-09T18:02:00Z
updated_at: 2026-04-09T18:13:02Z
---

The Clippy widget is awkwardly close to the bottom-right edge of the screen. Current CSS: bottom:44px; right:8px. Move it more toward center by increasing right offset (e.g. right:60px or 80px) so it feels less cramped.

## Summary of Changes

Changed .clippy-widget right from 8px to 60px in styles/main.css.

**Follow-up fix**: Also increased bottom from 44px to 100px.
