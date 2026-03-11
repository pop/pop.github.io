---
# editor-qc6i
title: Add image HTML utilities to models/post.rs
status: completed
type: task
priority: high
created_at: 2026-03-11T17:36:32Z
updated_at: 2026-03-11T17:36:32Z
---

Add pure utility functions to src/models/post.rs: (1) pub fn post_dir(path: &str) -> &str - extracts parent dir (same logic as private parent_dir in editor.rs line 902; also remove that private fn and update its 3 call sites to use post_dir from models). (2) pub fn extract_relative_image_srcs(html: &str) -> Vec<String> - scans <img tags for src= values not starting with http, //, /, or data:. (3) pub fn replace_image_srcs(html: &str, replacements: &HashMap<String, String>) -> String. (4) pub fn mime_type_for(path: &str) -> &'static str - maps png/jpg/jpeg/gif/webp/svg extensions. (5) pub fn bytes_to_data_url(bytes: &[u8], path: &str) -> String - produces data:{mime};base64,{encoded}.
