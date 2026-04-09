---
# project-os-qz60
title: Start menu height should match taskbar
status: completed
type: bug
priority: normal
created_at: 2026-04-09T18:01:58Z
updated_at: 2026-04-09T18:13:02Z
---

When clicking the Start button, the list of items should be taller to visually match the taskbar height. The start-menu.window currently has no minimum height set; it should fill or align with the 40px taskbar zone and/or have a taller minimum height.

## Summary of Changes

Increased .start-menu-item padding from 4px to 6px vertical. Added min-height: 120px to .start-menu.window.

**Follow-up fix**: Removed min-height (added dead space), increased item padding from 6px to 12px to match 40px taskbar row height.
