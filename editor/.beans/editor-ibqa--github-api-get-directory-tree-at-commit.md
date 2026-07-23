---
# editor-ibqa
title: GitHub API — get_directory_tree_at_commit
status: completed
type: feature
priority: high
created_at: 2026-03-19T03:29:18Z
updated_at: 2026-03-19T20:59:59Z
parent: editor-eb70
---

## What

Add a `get_directory_tree_at_commit` method to `GitHubClient` in `src/services/github.rs` that lists all blob (file) entries under a given directory prefix at a specific commit SHA.

This is used by the revert operation (T4) to enumerate which files existed in the post directory at a historical commit, so they can be compared with the current state and restored.

## Method signature

```rust
/// List all blob entries under `dir_prefix` in the repo tree at `commit_sha`.
/// Returns a flat list of (repo_path, blob_sha) pairs.
/// Pass dir_prefix as empty string "" for the root.
/// Uses the Git Trees API (REST) with recursive=1.
pub async fn get_directory_tree_at_commit(
    &self,
    commit_sha: &str,
    dir_prefix: &str,
) -> Result<Vec<(String, String)>, String>
```

Returns: `Vec<(path, blob_sha)>` where `path` is the full repo path (e.g. `"content/blog/my-post/image.png"`) and `blob_sha` is the git object SHA of that blob.

## Implementation

```rust
pub async fn get_directory_tree_at_commit(
    &self,
    commit_sha: &str,
    dir_prefix: &str,
) -> Result<Vec<(String, String)>, String> {
    let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/trees/{commit_sha}?recursive=1");
    let resp = self.get(&url).await?;

    match resp.status() {
        200 => {
            let tree: TreeResponse = resp.json().await.map_err(|e| e.to_string())?;
            let entries = tree
                .tree
                .into_iter()
                .filter(|te| {
                    te.entry_type == "blob"
                        && (dir_prefix.is_empty()
                            || te.path.starts_with(&format!("{dir_prefix}/"))
                            || te.path == dir_prefix)
                })
                .map(|te| (te.path, te.sha))
                .collect();
            Ok(entries)
        }
        401 => Err("Unauthorized \u2014 check your token".into()),
        404 => Err(format!("Commit not found: {commit_sha}")),
        status => Err(format!("GitHub API error: {status}")),
    }
}
```

## Notes

- `TreeResponse` and `TreeEntry` models already exist in `src/models/github.rs` — no new types needed.
- For standalone `.md` files (where `dir_prefix == ""`), the filter should match only the exact file path. But for this feature, the caller always passes `post_dir(path)` which is either an empty string (for top-level files) or the directory path. When dir_prefix is empty, all blobs are returned — the caller is responsible for filtering to just their file. See T4 for how this is handled.
- This method works with both commit SHAs and branch names as `commit_sha` since the Git Trees API accepts any git ref.
- Place in a new `// ── Tree operations` section near the other read methods, before `// ── Branch operations`.

## Files

- `src/services/github.rs` — add method only

## Validation

```bash
cargo check --target wasm32-unknown-unknown
cargo clippy
```

## Todo

- [x] Add `get_directory_tree_at_commit` to `GitHubClient`
- [x] Validate with `cargo check --target wasm32-unknown-unknown`

## Summary of Changes

Added `get_directory_tree_at_commit` to `GitHubClient` in a new `// ── Tree operations` section before branch operations. Uses the REST Git Trees API with `recursive=1`, filters by type `blob` and path prefix.
