---
# editor-kjn6
title: Files without draft key should show no icon, not published icon
status: completed
type: bug
priority: normal
created_at: 2026-03-11T17:36:30Z
updated_at: 2026-03-11T17:36:30Z
---

Standalone .md files (not in a folder) that have no 'draft' key in their frontmatter should display no icon. Currently they show the Published icon (🗞).

Root cause: detect_post_status() in dashboard.rs (~line 36):
  let is_draft = fields.iter().any(|(k, v)| k == "draft" && v == "true");
  if is_draft { PostStatus::Draft } else { PostStatus::Published }

This conflates 'draft = false' with 'no draft key' — both return Published. Need to add a third case: if the draft key is absent, return a NoKey variant (or reuse NoFrontmatter). Then update render_entry() (~line 1600) to not render an icon for that case.
