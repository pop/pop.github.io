---
# project-os-0h28
title: Bare relative icon paths rendered as text (is_url misclassification)
status: in-progress
type: bug
priority: high
created_at: 2026-07-27T03:23:47Z
updated_at: 2026-07-27T03:23:47Z
parent: project-os-qfcq
---

After switching portfolios.toml asset paths to bare relative filenames (for the /portfolios subpath), all icons rendered as literal text instead of images.

Root cause: each component's is_url(s) helper decided <img> vs emoji/text via s.starts_with("http") || s.starts_with("data:") || s.contains('/'). Old paths (/foo.png) contained '/'; new bare filenames (foo.png) did not, so they fell through to the emoji/text <span> branch. No 404s — the images were never requested.

Fix: added || s.contains('.') to is_url in all 5 components (game_icon, game_window, taskbar, clippy, start_menu) so a bare filename is treated as an image path; emoji (no dot) still render as text.

## Tasks
- [x] Add s.contains('.') to is_url in all 5 components
- [ ] Rebuild + repackage, re-upload portfolios.tar.zst
- [ ] Verify icons render as <img> (browser)
