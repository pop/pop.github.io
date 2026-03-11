---
# editor-cgbt
title: 'Phase 20: Feature — editor formatting toolbar buttons'
status: completed
type: feature
priority: high
created_at: 2026-03-11T17:36:29Z
updated_at: 2026-03-11T17:36:29Z
---

Add five Markdown formatting buttons to the editor toolbar: Bold (**), Italic (*), Strikethrough (~~), Inline code (`), Code block (```). Each wraps selected text or inserts placeholder syntax. Requires: apply_format helper reading textarea selection, five Callback<MouseEvent> closures, .format-buttons div in toolbar, CSS in main.css. See PLANNING.md Phase 20.
