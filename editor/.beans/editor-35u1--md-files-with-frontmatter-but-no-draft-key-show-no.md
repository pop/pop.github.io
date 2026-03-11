---
# editor-35u1
title: '*.md files with frontmatter but no draft key show no icon'
status: completed
type: bug
priority: high
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

## Problem

In `src/components/dashboard.rs`, `detect_post_status()` (lines 36–50) maps the absent `draft` key to `PostStatus::NoFrontmatter`, which renders no icon:

```rust
None => PostStatus::NoFrontmatter,
```

But Zola's default for a missing `draft` field is `draft = false` (i.e., **published**). So a post with valid frontmatter but no explicit `draft` key should show the Published icon (📰), not be treated as if it has no frontmatter at all.

## Expected behaviour

- `draft = true` → Draft icon (🌱)
- `draft = false` **or absent** → Published icon (📰)
- No frontmatter at all → No icon

## Actual behaviour

- `draft = true` → Draft icon (🌱) ✓
- `draft = false` → Published icon (📰) ✓
- **`draft` key absent** → **No icon** ✗ (wrongly treated as NoFrontmatter)

## Root cause

`detect_post_status()` distinguishes between "has frontmatter, no draft key" and "no frontmatter" only by the key's presence, not by whether frontmatter exists at all. It returns `PostStatus::NoFrontmatter` in both cases.

## Note

This bug is already fixed for **folder** entries (commit 915823c introduced inline logic for `folder_md_statuses` that treats absent `draft` as Published). The fix needs to be applied to the standalone-file path in `detect_post_status()` as well.

## Fix sketch

In `detect_post_status()`, change the `None` arm to return `PostStatus::Published` (since frontmatter is confirmed to exist by that point — the early return for missing frontmatter already handles the no-frontmatter case):

```rust
fn detect_post_status(content: &str) -> PostStatus {
    if extract_frontmatter(content).is_none() {
        return PostStatus::NoFrontmatter;
    }
    let fields = parse_frontmatter(content);
    match fields
        .iter()
        .find(|(k, _)| k == "draft")
        .map(|(_, v)| v.as_str())
    {
        Some("true") => PostStatus::Draft,
        _ => PostStatus::Published,   // absent or any non-"true" value → published
    }
}
```
