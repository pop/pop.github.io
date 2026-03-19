---
# editor-gtln
title: GitHub API — revert_directory_to_commit
status: completed
type: feature
priority: high
created_at: 2026-03-19T03:30:16Z
updated_at: 2026-03-19T21:02:08Z
parent: editor-eb70
blocked_by:
    - editor-ibqa
---

## What

Add a `revert_directory_to_commit` method to `GitHubClient` in `src/services/github.rs`. This is the core operation for the history revert feature.

Given a post file path, a target commit SHA (the historical state to restore), and the current editor branch name, it:
1. Enumerates files in the post directory at the target commit (historical state)
2. Enumerates files in the post directory at the current branch HEAD
3. Diffs the two: adds/restores files missing from current, deletes files added after the target, re-uploads files that changed
4. Creates all these changes as individual commits on the branch
5. Returns the final file SHA of the main `.md` file (for updating `file_sha` state in the editor)

## Method signature

```rust
/// Restore all files in the post directory to their state at `target_commit_sha`.
/// Writes commits directly to `branch`.
/// Returns the new SHA of the primary `post_path` file.
pub async fn revert_directory_to_commit(
    &self,
    post_path: &str,
    target_commit_sha: &str,
    branch: &str,
) -> Result<String, String>
```

## Algorithm

```rust
use crate::models::post::post_dir;
use std::collections::HashMap;

let dir = post_dir(post_path).to_string();
let is_standalone = dir.is_empty(); // top-level .md with no directory

// 1. Get tree at target commit (historical state)
let historical = self.get_directory_tree_at_commit(target_commit_sha, &dir).await?;
// Map: path -> blob_sha
let historical_map: HashMap<String, String> = if is_standalone {
    // Only care about the single file
    historical.into_iter().filter(|(p, _)| p == post_path).collect()
} else {
    historical.into_iter().collect()
};

// 2. Get tree at branch HEAD
let branch_head_sha = self.get_branch_sha(branch).await?;
let current = self.get_directory_tree_at_commit(&branch_head_sha, &dir).await?;
let current_map: HashMap<String, String> = if is_standalone {
    current.into_iter().filter(|(p, _)| p == post_path).collect()
} else {
    current.into_iter().collect()
};

// 3. Compute diff
// Files to restore: in historical but absent or changed in current
// Files to delete: in current but absent in historical
let to_restore: Vec<String> = historical_map
    .keys()
    .filter(|p| current_map.get(*p) != Some(&historical_map[*p]))
    .cloned()
    .collect();

let to_delete: Vec<String> = current_map
    .keys()
    .filter(|p| !historical_map.contains_key(*p))
    .cloned()
    .collect();

// 4. Delete files added after target (parallel would be ideal, sequential is safe)
for path in &to_delete {
    let sha = &current_map[path];
    let message = format!("Revert: remove {path}");
    self.delete_file(path, sha, &message, branch).await?;
}

// 5. Restore changed/missing files
// For each file: fetch bytes at target_commit_sha, re-upload to branch
for path in &to_restore {
    let bytes = self.get_file_bytes(path, target_commit_sha).await?;
    // Get current SHA on branch (if file exists there — needed for update vs create)
    let existing_sha = self.get_file(path, branch).await.ok().map(|f| f.sha);
    let message = format!("Restore version: {path}");
    self.upload_binary_file(path, &bytes, &message, existing_sha.as_deref(), branch).await?;
}

// 6. Return the new SHA of the primary file
let primary = self.get_file(post_path, branch).await?;
Ok(primary.sha)
```

## Important notes on the algorithm

- `get_file_bytes(path, commit_sha)` works because the Contents API accepts any git ref (including a commit SHA) as `?ref=`. Confirmed by existing usage patterns in this codebase.
- For text files like `.md`, using `upload_binary_file` is fine — it base64-encodes and uploads. For images it's required.
- The order of operations matters: delete first, then restore. This avoids SHA conflicts when a file existed at target, was deleted, and then re-added.
- If `to_restore` and `to_delete` are both empty, the post is already at the target state. In that case, return early with the current file SHA (no commits needed). Check: `if to_restore.is_empty() && to_delete.is_empty() { ... }`
- The commit message format for each individual file operation is per above. The overall "this is a revert" semantics come from the sequence of commits, not a single commit.
- Errors mid-way leave the branch in a partially-reverted state. This is acceptable — the user will see an error and can try again.
- Fetch the primary file SHA at the end (step 6) because the restore commits above may change it.

## Files to modify

- `src/services/github.rs` — add the method
- Add `use crate::models::post::post_dir;` import in the method body if not already in scope (it's available from `crate::models::post`)

## Existing method reference

Existing imports at top of `src/services/github.rs`:
```rust
use crate::models::post::{
    bytes_to_data_url, extract_relative_image_srcs, post_dir, replace_image_srcs,
};
```
`post_dir` is already imported. `get_file_bytes`, `get_file`, `delete_file`, `upload_binary_file`, `get_branch_sha` all already exist on `GitHubClient`.

## Validation

```bash
cargo check --target wasm32-unknown-unknown
cargo clippy
```

## Todo

- [x] Add `revert_directory_to_commit` to `GitHubClient`
- [x] Handle the early-return case when no files differ
- [x] Validate with `cargo check --target wasm32-unknown-unknown`

## Summary of Changes

Added `revert_directory_to_commit` to `GitHubClient` in a new `// ── Revert operations` section. Diffs historical vs current tree, deletes surplus files, restores missing/changed files, returns new primary file SHA.
