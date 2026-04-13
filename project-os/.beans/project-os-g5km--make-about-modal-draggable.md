---
# project-os-g5km
title: Make About modal draggable
status: completed
type: feature
priority: normal
created_at: 2026-04-11T02:32:34Z
updated_at: 2026-04-11T02:33:46Z
---

The About This Portfolio modal is currently a static centered dialog. It should be draggable by its title bar, like project windows. Implementation: add modal_pos state + drag listeners in clippy.rs, replace backdrop+centering with position:fixed and transform:translate(-50%,-50%) default, override with explicit coords when dragged. Add onmousedown to title bar.

## Summary of Changes

Added modal_pos state + _modal_move_listener/_modal_up_listener + modal_ref in clippy.rs. Added onmousedown_modal callback (same getBoundingClientRect pattern as widget drag). Updated open_modal to reset modal_pos to None on each open. Replaced clippy-modal-backdrop wrapper with position:fixed + transform:translate(-50%,-50%) default, overridden with explicit coords when dragged. Removed unused .clippy-modal-backdrop CSS.
