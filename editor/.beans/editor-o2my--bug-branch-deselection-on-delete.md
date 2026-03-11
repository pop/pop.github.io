---
# editor-o2my
title: 'Bug: branch deselection on delete'
status: completed
type: bug
priority: high
created_at: 2026-03-11T17:36:30Z
updated_at: 2026-03-11T17:36:30Z
---

After Publish or Discard in the editor, the dashboard's active_branch state is not refreshed in the same render cycle. User sees errors and must manually click 'View source'.\n\nFix:\n- Investigate on_confirm_publish and on_discard callbacks in components/dashboard.rs (approx lines 645–721)\n- If branch selector panel is open during deletion, close it and reset panel state\n- Ensure force_refresh counter is incremented after set_active_branch so refetch uses source branch\n- Verify: deleting a branch returns to source-branch content with no errors
