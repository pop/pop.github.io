---
# project-os-v7k4
title: Move demo video to right of project content
status: completed
type: task
priority: normal
created_at: 2026-04-13T21:42:11Z
updated_at: 2026-04-13T21:42:41Z
---

Change GameWindow layout so demo video/image appears to the right of other content (title, tech stack, description, launch button) instead of below it.

## Summary of Changes\n\nWrapped left-side content (title, contributors, tech stack, description, launch button) in a `.game-window-left` div. Made `.game-window-content` a flex row. Demo video/image now renders to the right as a flex sibling with `flex-shrink: 0` and a fixed `max-width: 240px`.
