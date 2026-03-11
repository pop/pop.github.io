---
# editor-uz8o
title: 'Optimization: GraphQL query batching opportunities'
status: completed
type: task
priority: low
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

Future optimization: batch multiple GraphQL reads into single queries.\n\n- Editor load: batch get_branch_sha + get_file into a single query\n- Dashboard sort-by-date: batch directory listing + per-file last-commit-date into one query using history(first: 1) on each tree entry\n- Branch selector + CI status: batch branch list + check suite status per branch
