---
# project-os-wmw7
title: Remember dismissed About modal in localStorage
status: completed
type: feature
priority: normal
created_at: 2026-04-11T02:30:43Z
updated_at: 2026-04-11T02:33:46Z
---

The Clippy About modal opens on every page load (clippy.rs:22 initializes modal_open to true). It should remember if the user has dismissed it and not reopen. Implementation: use web_sys window().local_storage() to check/set a dismissed flag on close. Initialize use_state with !has_dismissed_about() instead of true.

## Summary of Changes

Added has_dismissed_about() and set_dismissed_about() helpers in clippy.rs using web_sys localStorage. Initialized modal_open with !has_dismissed_about() instead of true. Calls set_dismissed_about() on close.
