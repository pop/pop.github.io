---
# project-os-frqz
title: Move demo video to right of project content
status: completed
type: task
priority: normal
created_at: 2026-04-13T21:42:09Z
updated_at: 2026-04-13T21:47:28Z
---

Change GameWindow layout so demo video/image appears to the right of other content (title, tech stack, description, launch button) instead of below it.

## Summary of Changes\n\nWrapped left-side content in .game-window-left div, made .game-window-content a flex row. Demo video/image now renders to the right as a flex sibling with flex-shrink: 0 and max-width: 240px.
