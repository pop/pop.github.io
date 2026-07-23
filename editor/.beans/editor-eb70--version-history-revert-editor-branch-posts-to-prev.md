---
# editor-eb70
title: 'Version history: revert editor branch posts to previous commits'
status: completed
type: epic
priority: high
created_at: 2026-03-19T03:27:15Z
updated_at: 2026-03-19T21:10:22Z
---

## Overview

Allow users editing a post on an editor branch to view the commit history for that post, preview any previous version, and revert to it. The revert creates a new commit (non-destructive). Media/blobs in the post directory are restored alongside the markdown file. Cancelling a revert also creates a new commit, rolling back to the pre-revert state.

## Scope

- Editor branch posts only (feature is hidden / greyed-out when no active editor branch)
- History button appears in the editor toolbar, greyed out with tooltip until first save
- Selecting a version creates a revert commit immediately, switches to Split view, shows a confirm/cancel banner
- Confirming exits preview mode (commit stays)
- Cancelling creates a restore-to-pre-revert commit

## Tickets

- T1: CommitSummary model types
- T2: GitHub API — list_commits_for_path
- T3: GitHub API — get_directory_tree_at_commit
- T4: GitHub API — revert_directory_to_commit
- T5: Editor history panel (display only)
- T6: Editor revert flow
- T7: CSS styles

## Summary of Changes

Implemented the full version history revert feature across 7 tickets:

- **editor-nsws**: Added `CommitSummary` and GraphQL envelope types to `src/models/github.rs`
- **editor-ibqa**: Added `get_directory_tree_at_commit` REST method to `GitHubClient`
- **editor-bqzl**: Added `list_commits_for_path` GraphQL method to `GitHubClient`
- **editor-gtln**: Added `revert_directory_to_commit` method that diffs and restores post directory
- **editor-67fw**: Added collapsible history panel UI to editor with fetch effect
- **editor-0fro**: Implemented full revert flow with unsaved-changes gate, preview mode, confirm/cancel
- **editor-x78j**: Added CSS for all history and revert UI components
