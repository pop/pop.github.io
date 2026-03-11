---
# editor-sj9y
title: 'Phase 16: Bug fix — branch deselection on delete'
status: completed
type: bug
priority: critical
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

When a branch is deleted via Publish or Discard in the editor, the dashboard should immediately deselect it and refresh from source. Currently the user sees 'fail to load' errors and must manually click 'View source' and reload. Fix: ensure set_active_branch.emit(None) fires before the content-loading effect re-runs, close the branch selector panel, and increment force_refresh so the refetch uses source branch. See PLANNING.md Phase 16.
