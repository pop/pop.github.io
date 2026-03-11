---
# editor-frp9
title: 'Phase 8d: Show commit count or last activity per branch in branch selector'
status: completed
type: feature
priority: low
created_at: 2026-03-11T17:36:30Z
updated_at: 2026-03-11T17:36:30Z
---

Currently the branch selector lists editor/* branches by name only. Consider fetching and displaying the last commit date or commit count for each branch so users can identify stale branches at a glance. This requires extra API calls (one per branch) and may be deferred if performance cost is too high.
