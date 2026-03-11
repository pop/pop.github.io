---
# editor-pfey
title: 'Bug: add taxonomies.tags comment to new post template'
status: completed
type: bug
priority: normal
created_at: 2026-03-11T17:36:29Z
updated_at: 2026-03-11T17:36:29Z
---

Include a commented-out taxonomies.tags line in the new-post TOML frontmatter template.\n\nIn components/editor.rs generate_template function (approx line 843), add after 'draft = true':\n  # taxonomies.tags = ["comics", "games", "backlog", "movies", "tv", "whats-good"]\n\nVerify: creating a new post shows the tags comment in the frontmatter
