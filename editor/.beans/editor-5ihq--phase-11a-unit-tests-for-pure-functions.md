---
# editor-5ihq
title: 'Phase 11a: Unit tests for pure functions'
status: completed
type: task
priority: high
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

Add #[cfg(test)] unit test modules for pure functions — no browser or WASM runtime needed. Targets:
- models/post.rs: strip_frontmatter (with/without frontmatter, empty, unclosed), render_markdown (basic markdown, GFM, frontmatter stripping)
- services/github.rs: decode_github_content (valid base64, whitespace/newlines, invalid, empty)
- components/editor.rs: slug_from_path, title_from_slug, sanitize_filename, char_pos_to_byte_offset, parent_dir, generate_template
- components/dashboard.rs: format_size (bytes, KB, MB boundaries)
Consider extracting testable pure functions into a shared utils.rs module to avoid pulling in Yew dependencies.
