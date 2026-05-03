---
# editor-js9z
title: Prompt compress-or-raw when uploading an image in the editor
status: todo
type: feature
priority: normal
created_at: 2026-05-02T23:35:46Z
updated_at: 2026-05-02T23:35:46Z
---

When the user uploads an image in the post editor (via the "Upload Image" button or drag-and-drop), intercept the upload if the file is a compressible raster image and ask whether to compress first or upload raw.

## Background

The editor's upload flow lives entirely in `src/components/editor.rs`. The relevant pieces:

- `upload_image: Callback<web_sys::File>` (line ~490) — the single entry point for both the button and drag-and-drop paths.
- Inside that callback, after sync validation (MIME, 10 MB cap), a `spawn_local` block: reads bytes via FileReader, creates/gets an editor branch, checks for an existing SHA, then calls `client.upload_binary_file(...)`.
- `on_file_selected` (line ~632) — calls `upload_image.emit(file)` on the hidden file input change event.
- `on_drop` (line ~665) — also calls `upload_image.emit(file)` after filtering for image MIME types.

The compress machinery already exists in `src/compress.rs`:
- `is_compressible_image(name: &str) -> bool` — true for jpg/jpeg/png/webp.
- `compress_image(bytes: &[u8], file_name: &str) -> Result<CompressResult, String>` (WASM-only async) — resizes to max 1000 px, re-encodes JPEG at 0.85 quality.
- `CompressResult { original_width, original_height, new_width, new_height, compressed_bytes, preview_data_url }`.

All the CSS for the compress preview UI (`.compress-preview`, `.compress-stats`, `.compress-confirm-btn`, `.compress-cancel-btn`, `.compress-modal-actions`, `.compress-size-warning`) is already defined in `styles/main.css` and is available app-wide.

## Decision scope

Only compressible images (JPEG, PNG, WebP) trigger the prompt. GIF and SVG bypass it and upload raw exactly as today.

## New state

Add to the editor component alongside the existing `uploading`/`error`/`save_msg` state:

```rust
let upload_pending = use_state(|| Option::<UploadPending>::None);
```

```rust
#[derive(Clone, PartialEq)]
enum UploadPending {
    AwaitingChoice {
        raw_bytes: std::rc::Rc<Vec<u8>>,
        file_name: String,
        upload_path: String,
    },
    Compressing,
    CompressPreview {
        raw_bytes: std::rc::Rc<Vec<u8>>,
        file_name: String,
        upload_path: String,
        original_size: u64,
        original_width: u32,
        original_height: u32,
        compressed_bytes: std::rc::Rc<Vec<u8>>,
        compressed_width: u32,
        compressed_height: u32,
        preview_data_url: String,
    },
}
```

Note: `upload_path` is already computed synchronously (`sanitize_filename` + `post_dir`), so it can be stored before going async. The branch creation and existing-SHA check remain deferred until the user confirms — same latency model as the current flow.

## Modified `upload_image` callback

After reading the bytes (async FileReader step), branch on `is_compressible_image`:

```
read bytes →
  if is_compressible_image(&file_name):
    upload_pending.set(Some(UploadPending::AwaitingChoice { raw_bytes: Rc::new(bytes), file_name, upload_path }))
    return   // do NOT upload yet
  else:
    // existing branch-create + SHA-check + upload_binary_file path unchanged
```

The `uploading` flag should NOT be set while waiting for user choice (the modal has its own visual state). Only set `uploading = true` when actual upload begins (after user confirms either path).

## New callbacks

### `on_upload_choose_raw`

Takes `raw_bytes`, `file_name`, `upload_path` from `AwaitingChoice` state. Clears `upload_pending`, sets `uploading = true`, then runs the existing branch-create + SHA-check + `upload_binary_file` logic with the raw bytes.

### `on_upload_choose_compress`

Takes `raw_bytes`, `file_name`, `upload_path` from `AwaitingChoice`. Sets `upload_pending` to `Compressing`, then `spawn_local`:
1. `crate::compress::compress_image(&raw_bytes, &file_name).await`
2. On Ok: set `upload_pending` to `CompressPreview { ... }` with all fields from `CompressResult` plus `original_size: raw_bytes.len() as u64`.
3. On Err: set `upload_pending` to `None`, set `error` to the message.

### `on_upload_compress_confirm`

Takes `compressed_bytes`, `file_name`, `upload_path` from `CompressPreview`. Clears `upload_pending`, sets `uploading = true`, then runs branch-create + SHA-check + `upload_binary_file` with the compressed bytes.

### `on_upload_cancel`

Sets `upload_pending = None`. Clears any `error` set during compression. Does NOT clear `uploading` (which should already be false at this point).

## Modal UI (render alongside existing error/save_msg display, below the toolbar)

The modal should appear as an overlay (`position: fixed; inset: 0; z-index: 100`) matching the dashboard modal pattern. It is separate from the editor's existing inline error messages.

### AwaitingChoice phase

```
┌─────────────────────────────────────────────────┐
│  Upload image                                   │
│  photo.jpg  (2.1 MB)                            │
│                                                 │
│  Compress first?  Images over 1000 px will be   │
│  resized and re-encoded at 85% quality.         │
│                                                 │
│       [ Compress & Preview ]  [ Upload Raw ]    │
└─────────────────────────────────────────────────┘
```

- "Compress & Preview" → `on_upload_choose_compress` (blue `.compress-confirm-btn`)
- "Upload Raw" → `on_upload_choose_raw` (neutral / default button)
- Clicking the overlay backdrop → `on_upload_cancel`

### Compressing phase

Show "Compressing…" text inside the modal.

### CompressPreview phase

Identical layout to the dashboard compress preview modal:
- Filename
- Stats table: original size → new size, original dimensions → new dimensions
- If `compressed_bytes.len() as u64 >= original_size`: render `<p class="compress-size-warning">` amber warning
- `<img src={preview_data_url} class="compress-preview" />`
- Action row (`class="modal-actions compress-modal-actions"`):
  - "Cancel" (`class="compress-cancel-btn"`) → `on_upload_cancel`
  - "Confirm & Upload" (`class="compress-confirm-btn"`) → `on_upload_compress_confirm`

## Files to change

- `src/components/editor.rs` — add `UploadPending` enum, `upload_pending` state, four new callbacks, restructure the `upload_image` callback, render modal HTML
- No changes needed to `src/compress.rs`, `styles/main.css`, or `src/services/github.rs`

## Tests

The compress logic and helpers are already tested. Add native unit tests for the one new pure-Rust decision:

```rust
// In src/components/editor.rs or a helper module, if extracted:
#[test]
fn compressible_images_trigger_choice() // jpg, jpeg, png, webp → is_compressible_image = true
#[test]
fn non_compressible_images_skip_choice() // gif, svg → is_compressible_image = false
```

These can be thin wrappers that just call `crate::compress::is_compressible_image` — their value is documenting the policy at the call site.

## Validation

```sh
cargo fmt
cargo check --target wasm32-unknown-unknown
cargo clippy
cargo test
```

All must pass. No `#[allow(dead_code)]`.
