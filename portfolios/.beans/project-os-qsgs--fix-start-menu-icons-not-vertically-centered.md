---
# project-os-qsgs
title: 'Fix: start menu icons not vertically centered'
status: completed
type: bug
priority: normal
created_at: 2026-04-10T21:54:21Z
updated_at: 2026-04-10T23:23:23Z
---

Icons in the start menu items are not vertically centered (top-aligned). They should be centered vertically alongside the item label text. See screenshot.

## Summary of Changes\n\nAdded display:flex + align-items:center + gap:8px to .start-menu-item in main.css. Added .start-menu-icon rule with width/height/object-fit/flex-shrink.
