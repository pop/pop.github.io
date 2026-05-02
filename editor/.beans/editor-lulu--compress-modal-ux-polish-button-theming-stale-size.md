---
# editor-lulu
title: 'Compress modal UX polish: button theming, stale size, warning color'
status: completed
type: bug
priority: high
created_at: 2026-05-02T23:23:08Z
updated_at: 2026-05-02T23:25:51Z
---

Three UX issues observed when using the compress image modal. All three are small, targeted fixes.

## Issue 1 — Button theming and layout

**Current:** The compress modal action row uses bare `<button>` (Cancel) and `class="compress-confirm-btn"` (Confirm & Upload). Neither class has CSS defined. `modal-actions` uses `justify-content: flex-end`, so buttons are right-aligned, not centered.

**Fix:**

In `styles/main.css`:
- Add `.compress-confirm-btn` as a solid blue primary button (use `#0366d6` / `#005cc5` hover), matching the padding/font-size of existing `.publish-btn` / `.delete-btn`.
- Add `.compress-cancel-btn` as a red ghost button identical in shape to the existing `.delete-btn` style.
- Add `.compress-modal-actions` (or `.compress-actions`) with `justify-content: center; gap: 1.25rem;` for even spacing.

In `src/components/dashboard.rs`, update the compress modal action row:
```rust
<div class="modal-actions compress-modal-actions">
    <button class="compress-cancel-btn" onclick={on_compress_cancel.clone()}>{"Cancel"}</button>
    <button class="compress-confirm-btn" onclick={on_compress_confirm.clone()}>
        {"Confirm \u{0026} Upload"}
    </button>
</div>
```

## Issue 2 — Original size is stale when re-compressing

**Current:** `on_compress_request` sets `original_size: entry.size` where `entry` is the `ContentEntry` captured from the directory listing at the time the user clicks Compress. If the listing was fetched before a previous compress+upload cycle completed (or GitHub's API has a brief propagation delay), `entry.size` is the pre-compression file size rather than the current on-disk size.

**Fix:** Use the actual downloaded bytes length instead of the metadata size:

In `src/components/dashboard.rs`, inside the `spawn_local` block of `on_compress_request`, change:
```rust
// BEFORE
original_size: entry.size,

// AFTER
original_size: bytes.len() as u64,
```

`bytes` is the `Vec<u8>` returned by `client.get_file_bytes(...)`, so it always reflects the actual content that was downloaded and compressed — never stale.

## Issue 3 — Compression warning is invisible

**Current:** When the compressed file is larger than the original, a `<p class="modal-warning">` is rendered. The existing `.modal-warning` rule uses `color: #666` (gray text on white), which blends in.

**Fix:** In `styles/main.css`, add a dedicated warning class for this specific message (keep `.modal-warning` unchanged so other modals are unaffected):

```css
.compress-size-warning {
    font-size: 0.85rem;
    color: #856404;
    background: #fff3cd;
    border: 1px solid #ffc107;
    border-radius: 4px;
    padding: 0.4rem 0.75rem;
    margin: 0 0 0.75rem;
}
```

In `src/components/dashboard.rs`, change the warning paragraph class:
```rust
// BEFORE
<p class="modal-warning">{"Warning: the compressed file is larger than the original. Consider cancelling."}</p>

// AFTER
<p class="compress-size-warning">{"Compressed file is larger than the original — consider cancelling."}</p>
```

## Files to change

- `styles/main.css` — add `.compress-confirm-btn`, `.compress-cancel-btn`, `.compress-modal-actions`, `.compress-size-warning`
- `src/components/dashboard.rs` — three targeted edits (action div class, `original_size` field, warning class)

## Tests

No new unit tests needed (pure CSS + single-field value change). Validate with:
```sh
cargo fmt
cargo check --target wasm32-unknown-unknown
cargo clippy
cargo test
```

All must pass.
