---
# project-os-uwqw
title: 'Clippy widget: rotating quotes + AI disclosure modal'
status: completed
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T18:10:24Z
parent: project-os-zsw7
---

Implement the Clippy widget in the bottom-right corner.
- src/components/clippy.rs: fixed position, bottom-right, above taskbar but below game windows (z-index: 10, windows start at 100+)
- Displays clippy.gif (placeholder) with a speech bubble showing current quote
- Quote rotates every 5s via gloo_timers Interval, cycles through config.quotes
- Clicking Clippy opens an AI disclosure modal (use win95.css dialog styles)
- Modal text: AI disclosure statement about the portfolio being built with Claude assistance
- Modal has OK button to close; modal z-index above Clippy but below game windows

## Summary of Changes

- Added PartialEq derive to Quote struct in src/config.rs (required for use in Yew props)
- Created src/components/clippy.rs: Clippy function component with rotating quotes via gloo_timers::callback::Interval every 5 s, AI disclosure modal toggled on icon click
- Fixed closure type mismatch in use_effect_with destructor by boxing both branches as Box dyn FnOnce
- Updated src/components/mod.rs to export pub mod clippy
- Updated src/app.rs to import and render Clippy with quotes prop above the taskbar
- Expanded .clippy-widget CSS and added .clippy-bubble, .clippy-icon, .clippy-modal-backdrop, .clippy-modal styles to styles/main.css
- cargo check --target wasm32-unknown-unknown passes cleanly
