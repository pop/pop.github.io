---
# editor-8fm1
title: Add dismiss button to version-restored notification
status: completed
type: feature
priority: normal
created_at: 2026-03-19T22:09:44Z
updated_at: 2026-03-19T22:11:48Z
parent: editor-eb70
---

After confirming a revert, the 'Version restored' save_msg banner has no way to be dismissed. Add an × close button so the user can clear it manually.


## Summary of Changes

Added on_dismiss_save_msg callback in editor.rs that sets save_msg to None on click. Changed banner from p to div with span for message and button for dismiss. Updated CSS to use display flex with space-between alignment and added save-msg-dismiss rules.
