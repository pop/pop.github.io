---
# editor-kwbn
title: Resolve images in Preview component
status: completed
type: task
priority: high
created_at: 2026-03-11T17:36:32Z
updated_at: 2026-03-11T17:36:32Z
blocked_by:
    - editor-9tyo
---

Update src/components/preview.rs to resolve images after markdown rendering. Add let rendered_html = use_state(String::new). In the content-loading spawn_local block, after content.set(text.clone()), add: let raw_html = render_markdown(&text); let resolved = client.resolve_images_in_html(&raw_html, &path, "source").await; rendered_html.set(resolved). In the render function (line 104), replace the inline render_markdown(&content) call with (*rendered_html).clone(). Update the syntax-highlighting use_effect_with to depend on rendered_html value instead of content.
