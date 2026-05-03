---
# editor-nsws
title: Add CommitSummary model types for history feature
status: completed
type: task
priority: high
created_at: 2026-03-19T03:27:46Z
updated_at: 2026-03-19T20:59:05Z
parent: editor-eb70
---

## What

Add new model types to `src/models/github.rs` to represent a commit entry in the post history panel.

## New types to add

```rust
/// A commit entry returned by the history query for a specific file path.
#[derive(Clone, Debug, PartialEq)]
pub struct CommitSummary {
    pub sha: String,
    pub message: String,
    pub date: String,       // ISO 8601, e.g. "2026-03-17T10:00:00Z"
    pub additions: u32,
    pub deletions: u32,
}
```

This type is returned by the planned `list_commits_for_path` service method (T2) and consumed by the editor history panel (T5). It does NOT need `Deserialize` — it is constructed from GraphQL response types.

Also add the GraphQL response envelope types needed to deserialize the GitHub history query response. The query will look like:

```graphql
query(: String!, nix-shell-env: String!, : String!, : String!) {
    repository(owner: , name: nix-shell-env) {
        ref(qualifiedName: ) {
            target {
                ... on Commit {
                    history(path: , first: 50) {
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
```

Add these serde-deserializable response types at the bottom of `src/models/github.rs` (after the existing GraphQL types section):

```rust
// ── History query types ──────────────────────────────────────────

#[derive(Deserialize)]
pub struct GqlHistoryData {
    pub repository: GqlRepoHistory,
}

#[derive(Deserialize)]
pub struct GqlRepoHistory {
    #[serde(rename = "ref")]
    pub git_ref: Option<GqlHistoryRef>,
}

#[derive(Deserialize)]
pub struct GqlHistoryRef {
    pub target: GqlHistoryTarget,
}

#[derive(Deserialize)]
pub struct GqlHistoryTarget {
    pub history: Option<GqlHistoryConnection>,
}

#[derive(Deserialize)]
pub struct GqlHistoryConnection {
    pub nodes: Vec<GqlHistoryNode>,
}

#[derive(Deserialize)]
pub struct GqlHistoryNode {
    pub oid: String,
    pub message: String,
    #[serde(rename = "committedDate")]
    pub committed_date: String,
    pub additions: u32,
    pub deletions: u32,
}
```

## Placement in file

Follow the existing pattern: struct definitions after the last existing GraphQL type block (currently ending around line 232 with `GqlRefNode`).

## Validation

```bash
cargo check --target wasm32-unknown-unknown
cargo clippy
```

No warnings, no dead code suppressions. If `CommitSummary` is not yet referenced by other code, add a `#[cfg(test)]` usage or leave it — the compiler may warn but do NOT add `#[allow(dead_code)]`.

Actually: since this ticket is depended on by T2 and T5, and those will add usages, it is fine if this produces dead-code warnings temporarily. Remove dead-code if the full feature has not yet been integrated.

## Files

- `src/models/github.rs` — only file to touch

## Todo

- [x] Add `CommitSummary` struct to `src/models/github.rs`
- [x] Add GraphQL response types (`GqlHistoryData`, `GqlRepoHistory`, `GqlHistoryRef`, `GqlHistoryTarget`, `GqlHistoryConnection`, `GqlHistoryNode`) to `src/models/github.rs`
- [x] Export types in the module (they are already public by default since the file is `pub mod`)
- [x] Validate with `cargo check --target wasm32-unknown-unknown`

## Summary of Changes

Added `CommitSummary` struct and GraphQL response envelope types (`GqlHistoryData`, `GqlRepoHistory`, `GqlHistoryRef`, `GqlHistoryTarget`, `GqlHistoryConnection`, `GqlHistoryNode`) to `src/models/github.rs`.
