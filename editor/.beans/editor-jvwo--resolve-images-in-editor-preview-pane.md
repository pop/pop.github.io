---
# editor-jvwo
title: Resolve images in Editor preview pane
status: completed
type: task
priority: high
created_at: 2026-03-11T17:36:32Z
updated_at: 2026-03-11T17:36:32Z
blocked_by:
    - editor-9tyo
---

Update src/components/editor.rs debounced render effect (lines 138-166) to call resolve_images_in_html. Clone auth.token, props.path, and auth.active_branch into the effect closure. Inside spawn_local after sleep_ms(200), create GitHubClient from token, compute branch = active_branch.unwrap_or("source"), call client.resolve_images_in_html(&raw_html, &path, &branch).await and set rendered_html to the result instead of raw render_markdown output.
