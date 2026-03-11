---
# editor-mt5a
title: 'Fix: deleting a file twice results in a 404 error'
status: completed
type: bug
priority: high
created_at: 2026-03-11T17:36:29Z
updated_at: 2026-03-11T17:36:29Z
---

delete_file() in services/github.rs only handles HTTP 200 and 401; a 404 (file already deleted) falls through to a cryptic generic error. Fix: add explicit 404 arm returning 'File not found — it may have already been deleted'. See PLANNING.md Phase 22.
