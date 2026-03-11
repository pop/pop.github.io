---
# editor-i0y9
title: 'Testing: extract pure functions into utils.rs module'
status: completed
type: task
priority: normal
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

Extract testable pure functions from component files into a shared utils.rs module to simplify testing and avoid pulling in Yew dependencies.\n\nFunctions to extract from components/editor.rs: slug_from_path, title_from_slug, sanitize_filename, char_pos_to_byte_offset, parent_dir, generate_template\nFunctions to extract from components/dashboard.rs: format_size
