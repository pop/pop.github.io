---
# editor-2jvv
title: 'fix: get_file silently returns empty content for images >1 MB, breaking previews'
status: completed
type: bug
priority: high
created_at: 2026-05-02T23:12:20Z
updated_at: 2026-05-02T23:14:42Z
---

## Problem

The GitHub Contents API returns `content: null` for files over 1 MB. `get_file_bytes` (added on this branch) already detects this and falls back to the Git Blob API. However, `get_file` and `get_file_rest` do **not** — they simply propagate `None` content. This silently produces empty bytes/text for every caller that unwraps `file.content.unwrap_or_default()`.

Two distinct preview code paths are affected.

---

## Broken Call Chains

### Path 1 — Editor in-line preview and Preview page (any image >1 MB in a post)

```
resolve_images_in_html()              github.rs:313
  └─ get_file_bytes()                 github.rs:338
       └─ [already fixed — falls back to get_blob_raw_bytes() when content is empty]
```

**This path is already fixed.** `resolve_images_in_html` calls `get_file_bytes`, which has the large-file fallback.

### Path 2 — Dashboard compress-image preview (`CompressPhase::Preview`)

```
on_compress_request callback          dashboard.rs:1307
  └─ get_file_bytes()                 github.rs:261
       └─ [already fixed — has blob fallback]
```

**This path is also already fixed** (it uses `get_file_bytes`).

### Path 3 — Broken: post status detection for .md files (not images, but same root cause)

Not an image-preview issue. Not in scope.

### Path 4 — BROKEN: `get_file` / `get_file_rest` used anywhere that expects binary bytes

`get_file` returns `FileContent { content: Option<String> }`. When the file is >1 MB, the REST API returns `content: null`, so after decoding `fc.content` is `None`. Callers that do `.unwrap_or_default()` get an empty string — no error, no signal.

Specific callers that could be hit with image blobs or large markdown:

| Location | Code path | What breaks |
|---|---|---|
| `editor.rs:804` | `client.get_file(&path, &branch).await` (after revert) | Large post content read back as empty |
| `editor.rs:882` | same pattern in revert-effect block | Same |
| `editor.rs:741` | `get_file` in discard-then-revert | Same |
| `preview.rs:42` | `client.get_file(&path, "source")` | Markdown file >1 MB renders as blank preview |
| `editor.rs:1181` (load_file) | `client.get_file(path, branch)` | Large .md file opened in editor appears blank |

All of these ultimately surface as a blank editor/preview with no error message shown.

---

## Root Cause

`get_file_rest` (github.rs:240–255) calls `resp.json()` into a `FileContent` struct. When GitHub returns `"content": null` for a >1 MB file, `fc.content` is `None`. The method returns `Ok(FileContent { content: None, ... })` without error. Every caller that does:

```rust
let text = file.content.unwrap_or_default();  // silently ""
```

gets an empty string — no error, no fallback.

`get_file_graphql` (github.rs:199–238) has the same flaw: GraphQL returns `text: null` for binary blobs (and possibly for large text files), so `blob.text` is `None`.

---

## Fix Strategy

### 1. Add a fallback in `get_file_rest` (github.rs:240)

After parsing `FileContent`, check if `content` is None or empty. If so and the file size is >1 MB (or the `sha` is present and content is empty), fall back to `get_blob_raw_bytes` and decode the raw bytes as UTF-8.

Pattern to mirror from `get_file_bytes`:

```rust
async fn get_file_rest(&self, path: &str, branch: &str) -> Result<FileContent, String> {
    let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref={branch}");
    let resp = self.get(&url).await?;

    match resp.status() {
        200 => {
            let mut fc: FileContent = resp.json().await.map_err(|e| e.to_string())?;
            fc.content = fc.content.map(|c| decode_github_content(&c));
            // Large-file fallback: content is null for files >1 MB
            if fc.content.as_deref().unwrap_or("").is_empty() && !fc.sha.is_empty() {
                let raw_bytes = self.get_blob_raw_bytes(&fc.sha).await?;
                fc.content = Some(String::from_utf8_lossy(&raw_bytes).to_string());
            }
            Ok(fc)
        }
        // ... existing error arms
    }
}
```

Note: `get_blob_raw_bytes` requires a token (`require_token`). If the client is anonymous and the file is large, this fallback will fail with an auth error. That is acceptable — an anonymous user cannot read large files from a private API anyway. The error should propagate (it already does via `?`).

### 2. Validate `get_file_graphql` (github.rs:199)

GraphQL returns `text: null` for binary blobs. This is unlikely to be an issue for .md files (which are text) but worth noting. No code change is needed here unless a bug is confirmed — the `blob.text` field maps to `Option<String>`, and callers already handle `None` via `unwrap_or_default`. However the GraphQL path never calls `get_blob_raw_bytes` either, so if a text file somehow exceeds the GraphQL inline limit, it would also silently return empty. Add a comment noting this limitation.

---

## Tests to Add

### Native unit tests (`src/services/github.rs`, `#[cfg(test)]` module)

The existing tests only cover `decode_github_content`. The HTTP layer cannot be tested without mocking (see `tests/wasm.rs` preamble). However, a regression-guard comment should be added near the fallback path explaining the large-file scenario.

### WASM integration tests (`tests/wasm.rs`)

Cannot mock fetch without significant refactoring (documented in the wasm.rs preamble). No new WASM tests are practical here.

### Manual validation

1. Find or create a test image in the repo that is >1 MB.
2. Open a post that references it in the editor Preview mode — confirm the image renders.
3. Navigate to the Preview page (/preview) for that post — confirm the image renders.
4. Open a post whose .md file is >1 MB in the editor — confirm text appears (not blank).

---

## Validation Commands

```bash
cargo fmt
cargo check --target wasm32-unknown-unknown
cargo clippy --target wasm32-unknown-unknown
cargo test
```
