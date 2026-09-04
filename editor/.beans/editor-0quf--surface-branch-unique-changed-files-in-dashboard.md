---
# editor-0quf
title: Surface branch-unique changed files in dashboard
status: completed
type: feature
priority: high
created_at: 2026-08-20T18:05:46Z
updated_at: 2026-08-20T18:17:52Z
---

When an editor branch is selected, the dashboard file list looks identical to source — there is no way to tell which files were added or edited on the branch.

## Plan
- [x] Fetch `compare_branches(source, active_branch)` when the active branch changes (and on refresh)
- [x] Add a "Changed on this branch" panel listing the changed files, clickable to open in the editor
- [x] Badge entries in the normal directory listing (A/M/D/R for files, changed-count for folders)
- [x] Add CSS for the panel and badges
- [x] Validate with cargo fmt / check / clippy / test

## Summary of Changes

When an editor branch is active, the dashboard now compares it against `source` and shows what the branch touches.

- `src/components/dashboard.rs`: added `ChangeKind` / `BranchChange` plus `change_for_path` and `changes_under_dir` helpers; a `use_effect_with((active_branch, force_refresh))` that calls `compare_branches(source, branch)`; `render_branch_changes` (collapsible panel listing every changed file with an A/M/D/R badge and +/- stats, clickable to open in the editor); change badges in `render_entry` and in the global search rows; an `.is-changed` row tint. Directories get an "N changed" pill when they contain changed files.
- `styles/main.css`: styles for the panel, the badges, and the folder counts.
- 4 unit tests for the path helpers (63 native tests pass).

Deleted files only appear in the panel — they are gone from the tree, so the listing cannot flag them.
