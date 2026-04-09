---
# project-os-zv5e
title: Clippy bubble dismisses on click-outside, restores on re-click or 60s
status: completed
type: feature
created_at: 2026-04-09T18:27:03Z
updated_at: 2026-04-09T18:27:03Z
---

Document-level click listener hides the bubble when clicking outside .clippy-widget. Listener is paused while modal is open. Clicking the icon while bubble is hidden restores it immediately. 60s timer auto-restores if not clicked.
