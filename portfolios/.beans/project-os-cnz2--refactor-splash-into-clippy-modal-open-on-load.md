---
# project-os-cnz2
title: 'Refactor: splash into Clippy modal (open on load)'
status: completed
type: task
priority: normal
created_at: 2026-04-10T23:24:50Z
updated_at: 2026-04-10T23:25:57Z
---

Remove the Splash window component. Move the PortfoliOS logo and brand title into the Clippy 'About' modal header. Open the modal by default on page load.

## Summary of Changes\n\nRemoved Splash component entirely. Moved brand_title and logo into ClippyConfig. Modal opens by default on load. Logo + title shown as header row inside the modal body.
