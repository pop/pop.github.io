---
# project-os-rhnf
title: 'Mobile UX: draggable windows, start menu sizing, video size, Clippy visibility'
status: completed
type: task
priority: normal
created_at: 2026-04-13T22:57:19Z
updated_at: 2026-04-13T22:58:45Z
---

Fix 4 mobile UX issues: (1) remove full-screen window override so touch drag works, (2) fix start menu taking full screen, (3) cap video size to avoid pixelated 144p, (4) show Clippy on mobile with % positioning

## Summary of Changes

CSS-only changes in styles/main.css:
- Replaced mobile .window full-screen override (position:fixed; inset:0) with max-width:95vw — unblocks existing touch drag handlers in window.rs
- Added .window .window-body overflow-y:auto on mobile for scrollable content
- Capped .game-demo at max-width:240px / max-height:180px on mobile (was 100% width)
- Changed .clippy-widget mobile from display:none to bottom:10vh; right:5vw
- Removed min-width:400px from global .window rule (start menu was unnecessarily wide)
