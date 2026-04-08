---
# project-os-uwqw
title: 'Clippy widget: rotating quotes + AI disclosure modal'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Implement the Clippy widget in the bottom-right corner.
- src/components/clippy.rs: fixed position, bottom-right, above taskbar but below game windows (z-index: 10, windows start at 100+)
- Displays clippy.gif (placeholder) with a speech bubble showing current quote
- Quote rotates every 5s via gloo_timers Interval, cycles through config.quotes
- Clicking Clippy opens an AI disclosure modal (use win95.css dialog styles)
- Modal text: AI disclosure statement about the portfolio being built with Claude assistance
- Modal has OK button to close; modal z-index above Clippy but below game windows
