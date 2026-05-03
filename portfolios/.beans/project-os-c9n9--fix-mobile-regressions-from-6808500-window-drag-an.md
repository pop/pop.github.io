---
# project-os-c9n9
title: 'Fix mobile regressions from 6808500: window drag and modal centering'
status: scrapped
type: bug
priority: normal
created_at: 2026-04-24T22:50:05Z
updated_at: 2026-04-24T22:59:26Z
---

Commit 6808500 introduced two regressions: (1) mobile windows are not draggable — CSS !important on left/top in the mobile .window rule overrides JS-set drag positions; (2) Clippy About modal is not centered on mobile — the .clippy-modal.window override was removed. Fix both.

## Reasons for Scrapping\n\nDuplicate of project-os-658w (created twice due to first create call returning null).
