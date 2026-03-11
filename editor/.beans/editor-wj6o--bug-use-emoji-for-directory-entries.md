---
# editor-wj6o
title: "Bug: use \U0001F4C2 emoji for directory entries"
status: completed
type: bug
priority: normal
created_at: 2026-03-11T17:36:29Z
updated_at: 2026-03-11T17:36:29Z
---

Replace the current ▸ (U+25B8) triangle with 📂 emoji for directory entries in the dashboard.\n\nIn components/dashboard.rs render_entry (approx line 1345), replace "\u{25B8}" with "📂".\n\nVerify: directory rows show 📂 and file rows continue to show ·
