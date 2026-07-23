---
# project-os-edb0
title: 'Splash screen: render as desktop window, close on click-outside'
status: completed
type: task
priority: normal
created_at: 2026-04-10T21:54:19Z
updated_at: 2026-04-10T23:23:23Z
---

Instead of a full-screen overlay, render the PortfoliOS splash screen as a standard desktop Window component. It should auto-close when the user clicks outside of it (click-outside dismissal, same pattern as the Clippy bubble).

## Summary of Changes\n\nRewrote splash.rs to render as a draggable Win95 Window component instead of a full-screen overlay. Uses a document-level click listener (same pattern as Clippy) to dismiss when clicking outside. Removed all splash-specific CSS from main.css.
