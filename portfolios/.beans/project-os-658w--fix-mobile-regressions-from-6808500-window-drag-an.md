---
# project-os-658w
title: 'Fix mobile regressions from 6808500: window drag and modal centering'
status: completed
type: bug
priority: normal
created_at: 2026-04-24T22:50:11Z
updated_at: 2026-04-24T22:59:38Z
---

Commit 6808500 introduced two regressions: (1) mobile windows are not draggable — CSS !important on left/top in the mobile .window rule overrides JS-set drag positions; (2) Clippy About modal is not centered on mobile — the .clippy-modal.window override was removed. Fix both.

## Summary of Changes\n\n- **main.css**: Removed `left: 18vw !important` and `top: 8px !important` from mobile `.window` rule so JS-set drag positions take effect. Restored `.clippy-modal.window` override with `left:50% !important; top:50% !important; transform:translate(-50%,-50%) !important; width:80vw !important` to center the About modal on mobile.\n- **state.rs**: `WindowManager::new` now accepts `is_mobile: bool`; mobile windows start at `(40, 8)` instead of the index-based desktop stagger (which would put higher-index windows off-screen on narrow viewports).\n- **app.rs**: Detects mobile viewport (`window.innerWidth <= 768`) before `use_state` and passes it to `WindowManager::new`.\n- **window.rs**: Replaced `use_effect_with((title_bar_ref, pos), ...)` with `use_mut_ref` for pos (always-current value in closures) and `use_effect_with(title_bar_ref, ...)` so the touchstart listener registers once and is never re-created during a drag.
