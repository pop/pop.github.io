---
# project-os-7uyo
title: 'fix: clippy bubble white-on-white text'
status: completed
type: bug
priority: high
created_at: 2026-04-08T18:31:07Z
updated_at: 2026-04-08T18:32:44Z
parent: project-os-zsw7
---

The .clippy-bubble CSS rule has no explicit color set. The speech bubble background is white but text inherits an unset/white color, making it invisible. Fix: add color: #000000 to .clippy-bubble in styles/main.css.

## Summary of Changes\n\nAdded color: #000000 to .clippy-bubble in styles/main.css.
