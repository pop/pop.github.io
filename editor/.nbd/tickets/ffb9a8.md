+++
title = "Phase 19: Bug fix — include tags in new post template"
priority = 5
status = "done"
ticket_type = "bug"
dependencies = []
+++
Add a commented-out taxonomies.tags line to the generate_template() function in components/editor.rs so new posts have example tags ready to uncomment. Example tags: comics, games, backlog, movies, tv, whats-good. See PLANNING.md Phase 19.