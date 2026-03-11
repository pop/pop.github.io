---
# editor-k58t
title: 'Fix: deleted files still appear in cache after deletion'
status: completed
type: bug
priority: high
created_at: 2026-03-11T17:36:29Z
updated_at: 2026-03-11T17:36:29Z
---

invalidate_all_caches() in dashboard.rs only clears dir_cache_* keys. The global all_files_index / all_files_index_{branch} cache is not cleared on deletion, so deleted files can reappear. Fix: extend the key-scan loop in invalidate_all_caches() to also remove keys starting with 'all_files_index'. See PLANNING.md Phase 22.
