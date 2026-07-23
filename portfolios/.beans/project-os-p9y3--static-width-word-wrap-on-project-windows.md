---
# project-os-p9y3
title: Static width + word-wrap on project windows
status: completed
type: feature
priority: normal
created_at: 2026-04-11T02:30:43Z
updated_at: 2026-04-11T02:33:46Z
---

Project windows can grow unboundedly wide when description text is long. They should have a fixed/static width so text word-wraps instead. Implementation: change window inline style in window.rs from min-width:400px to a static width (e.g. width:480px), and add overflow-wrap:break-word to .description in main.css.

## Summary of Changes

Changed window inline style in window.rs from min-width:400px to width:480px. Added overflow-wrap:break-word to .description in main.css.
