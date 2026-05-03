---
# project-os-kiq6
title: Allow Windows to be dragged freely over desktop icons
status: completed
type: feature
priority: normal
created_at: 2026-04-23T20:52:05Z
updated_at: 2026-04-23T21:05:15Z
blocked_by:
    - project-os-whzk
---

## Description

Windows should be draggable anywhere on the viewport, including over the desktop project icons. Currently on mobile the window is pinned by CSS so dragging leftward never covers the icon column. On desktop, windows already cover icons visually (icon-grid has no explicit z-index; windows start at `z_counter` = 100+).

## Current behavior

- **Desktop (>768px):** Drag has no Rust-side or CSS-side clamp. `src/app.rs` `on_move` writes `e.client_x() - offset` straight into `WindowManager.pos`. Icon-grid has no z-index, so windows already render over icons. No change required here — verify during testing.
- **Mobile (<=768px):** `styles/main.css:333-343` pins `.window` with `left: 18vw !important;` and `top: 8px !important;`. The `18vw` was deliberately chosen to reserve the icon column, so dragging can never cover the icons. The `!important` also defeats the inline `style` written by drag. See `project-os-whzk` for the shared root cause.

## Proposed approach

Covered by `project-os-whzk` at the CSS level (removing `left: 18vw !important;` and `top: 8px !important;`). This ticket ensures the outcome — that windows render unrestricted over icons — is explicitly verified, and adds follow-up polish if needed:

- If testing on narrow viewports shows initial windows spawn fully offscreen, adjust `src/state.rs:23` default `pos` stagger (e.g. viewport-aware offset) so the title bar is reachable on first open.
- If windows can be dragged fully offscreen (title bar unreachable, cannot drag back), add a very loose clamp in `app.rs`'s `on_move` keeping ~40px of title bar onscreen. The user requested total freedom, so only add this if empirically needed.

No other constraint code exists to remove.

## Tasks

- [x] Confirm `project-os-whzk` CSS changes land first (blocker)
- [ ] Verify desktop: drag a window fully over the icon column; confirm it covers icons and can be dragged back
- [ ] Verify mobile: drag a window over the icon column and to all four edges of the viewport
- [ ] If initial window pos on narrow viewports lands title bar offscreen, adjust `src/state.rs` stagger
- [ ] If windows can be dragged into an unrecoverable state, add a minimal title-bar-onscreen clamp in `src/app.rs` `on_move`
- [x] `cargo check --target wasm32-unknown-unknown` / `clippy`
- [ ] `trunk serve` smoke test desktop + mobile

## Testing notes

- Desktop: drag a window over the icon column, release, re-grab, drag back. Try dragging past left/top/right viewport edges.
- Mobile (devtools touch emulation + real device if possible): same flow. Verify icons are fully coverable. Verify Clippy modal and Start menu still open correctly.

## Related

- `project-os-whzk` — mobile touch drag CSS fix that produces this behavior as a side effect

## Summary of Changes

kiq6 is implicitly satisfied by `project-os-whzk` (commit c8aa07d). No code changes were made in this ticket.

**Verification findings:**

- `styles/main.css` mobile `.window` rule (lines 332-338): `left: 18vw !important;` and `top: 8px !important;` are confirmed absent. The rule only sets `position: fixed !important;`, `width: 80vw !important;`, `min-width/min-height: unset`, and `z-index: 500`. No CSS clamp on position remains.
- `src/app.rs` `on_move`: passes `(client_x - offset, client_y - offset)` directly to `win.pos` with no clamping — total freedom confirmed.
- `src/components/window.rs` drag handlers (mouse + touch): both compute `(client_x/touch_x - offset, client_y/touch_y - offset)` and emit via `on_move_mv` with no bounds check.
- `src/state.rs` initial pos: `(50 + i*30, 50 + i*30)` — up to 4 windows spread at x=50/80/110/140, y=50/80/110/140. At 80vw (≈300px on 375px phone), x=140 overflows the right edge but the title bar remains at the left, so the window is still grabbable. No adjustment needed.
- Grep for `clamp`, `.min(`, `.max(` across all `src/` Rust files: only hit is `webamp.rs:140` (`.max(0)` for Webamp initial centering, unrelated to draggable Window).

**Decisions:**
- Initial pos adjustment: skipped — title bar at x=50 is always onscreen on any reasonable viewport.
- Drag clamp: skipped per user preference for total freedom; no empirical evidence of unrecoverable state without a browser.
- Browser smoke tests (`trunk serve` desktop + mobile) left unchecked — require a live browser and are outside agent scope.
- `cargo fmt` drift: pre-existing across multiple files (not introduced by kiq6 or whzk); not committed to avoid unrelated churn.
