---
# editor-peio
title: Add word and character counter to editor and preview
status: completed
type: feature
priority: normal
created_at: 2026-07-12T03:43:36Z
updated_at: 2026-07-12T03:48:39Z
---

Show a live word and character count in both the editor and preview windows. Count should be based on rendered prose (not raw markdown syntax) so it reflects what a reader actually sees.

## Tasks
- [x] Add counter display to preview window (rendered prose)
- [x] Add live counter to editor window (updates as user types)
- [x] Style the counter unobtrusively (small text near header/footer)
- [x] Test with posts of varying length (unit tests + native cargo test)

## Summary of Changes

- Added count_prose() helper in src/models/post.rs that renders markdown, strips HTML tags/entities, and returns (words, characters) — frontmatter excluded, whitespace collapsed.
- Added 4 unit tests covering empty content, frontmatter exclusion, markdown-syntax stripping, and multi-block whitespace.
- Wired counter into preview.rs (as a <p class="prose-count"> under the header) and editor.rs (as a <span class="prose-count"> in .editor-meta, live-updated on keystroke via existing content state).
- Added .prose-count CSS in styles/main.css (muted color, 0.85rem).

## Not Done

- No in-browser verification (no browser tool available in this session); logic is covered by unit tests, but the visual placement/styling on the preview and editor pages should be checked in `trunk serve` before deploy.
