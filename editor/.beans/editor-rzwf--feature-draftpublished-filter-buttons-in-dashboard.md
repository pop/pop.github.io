---
# editor-rzwf
title: 'Feature: draft/published filter buttons in dashboard'
status: completed
type: feature
priority: normal
created_at: 2026-03-11T17:36:29Z
updated_at: 2026-03-11T17:36:29Z
---

Add All/Draft/Published filter buttons to dashboard alongside search and sort.\n\nImplementation:\n- Add StatusFilter enum (All, Draft, Published) and status_filter: use_state(StatusFilter::All) to dashboard\n- Extend display_entries filtering block (approx line 970) to skip .md files whose PostStatus doesn't match\n- Add filter button group (All / Draft / Published) to .filter-sort-bar (approx line 1113)\n- Add CSS for .status-filter-bar, .status-btn, .status-btn.active in styles/main.css\n- Reset status_filter to All on directory navigation\n- Verify: Draft shows only 🌱 files; Published shows only 📰 files; All restores full list
