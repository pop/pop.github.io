---
# editor-bqzl
title: GitHub API — list_commits_for_path
status: completed
type: feature
priority: high
created_at: 2026-03-19T03:28:06Z
updated_at: 2026-03-19T21:01:11Z
parent: editor-eb70
blocked_by:
    - editor-nsws
---

## What

Add a `list_commits_for_path` method to `GitHubClient` in `src/services/github.rs` that returns the commit history for a specific file path on an editor branch, including +/- diff stats per commit.

## Method signature

```rust
/// List commits on `branch` that touched `path`, up to 50 most recent.
/// Returns commit SHA, message, date, and additions/deletions counts.
/// Requires authentication (GraphQL).
pub async fn list_commits_for_path(
    &self,
    path: &str,
    branch: &str,
) -> Result<Vec<CommitSummary>, String>
```

## Implementation

Use the `self.graphql()` helper (already present in the file). The query:

```rust
let qualified = format!("refs/heads/{branch}");

let query = r#"
    query($owner: String!, $name: String!, $branch: String!, $path: String!) {
        repository(owner: $owner, name: $name) {
            ref(qualifiedName: $branch) {
                target {
                    ... on Commit {
                        history(path: $path, first: 50) {
                            nodes {
                                oid
                                message
                                committedDate
                                additions
                                deletions
                            }
                        }
                    }
                }
            }
        }
    }
"#;

let data: GqlHistoryData = self
    .graphql(
        query,
        json!({ "owner": OWNER, "name": REPO, "branch": qualified, "path": path }),
    )
    .await?;
```

Then map `GqlHistoryNode` → `CommitSummary`:

```rust
let nodes = data
    .repository
    .git_ref
    .and_then(|r| r.target.history)
    .map(|h| h.nodes)
    .unwrap_or_default();

Ok(nodes
    .into_iter()
    .map(|n| CommitSummary {
        sha: n.oid,
        message: n.message,
        date: n.committed_date,
        additions: n.additions,
        deletions: n.deletions,
    })
    .collect())
```

## Imports to add

Add to the import at the top of `src/services/github.rs`:

```rust
use crate::models::github::{
    // existing imports ...
    CommitSummary, GqlHistoryData,
};
```

## Notes

- This method requires a token (GraphQL only). It calls `self.graphql()` which internally calls `self.require_token()`, so no extra guard needed.
- Place the method in the `// ── Commit dates` section or create a new `// ── Commit history` section after it.
- `additions`/`deletions` on GitHub's GraphQL `Commit` type reflect the total changes across all files in that commit. On an editor branch (which only modifies one post's directory), this is a good proxy for the per-post diff.

## Files

- `src/services/github.rs` — add method
- `src/models/github.rs` — add imports (already modified by T1)

## Validation

```bash
cargo check --target wasm32-unknown-unknown
cargo clippy
```

## Todo

- [x] Add `list_commits_for_path` to `GitHubClient` in `src/services/github.rs`
- [x] Add `CommitSummary` and `GqlHistoryData` to the import list in `src/services/github.rs`
- [x] Validate with `cargo check --target wasm32-unknown-unknown`

## Summary of Changes

Added `list_commits_for_path` to `GitHubClient` in a new `// ── Commit history` section. Uses GraphQL history query to fetch up to 50 commits touching a path on a branch, mapping to `CommitSummary`. Added `CommitSummary` and `GqlHistoryData` imports.
