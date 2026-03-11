---
# editor-jsz9
title: 'Phase 12e: GraphQL batching optimizations'
status: completed
type: feature
priority: low
created_at: 2026-03-11T17:36:30Z
updated_at: 2026-03-11T17:36:30Z
---

Future optimization: batch multiple GraphQL queries into single round-trips where possible.
- Editor load: batch get_branch_sha + get_file into a single query (saves one round-trip on every editor open)
- Dashboard sort-by-date: batch directory listing + per-file last-commit-date using history(first: 1) on each tree entry's path (eliminates N parallel REST calls for N files)
- Branch selector + CI status: batch branch list + check suite status per branch
