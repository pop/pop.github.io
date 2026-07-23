---
# editor-bsys
title: Implement client-side image compression in the dashboard directory listing
status: completed
type: feature
priority: high
created_at: 2026-05-02T22:30:24Z
updated_at: 2026-05-02T22:54:19Z
---

## Overview

Add a "Compress" button to image entries in the dashboard directory listing. When clicked:
1. Download the image from GitHub
2. Compress/resize client-side using the browser Canvas API (via web-sys — zero new Rust deps)
3. Show a summary modal (old → new file size, old → new dimensions, preview of result)
4. On confirm: upload the compressed image, replacing the old file

Button is only visible to authenticated users.

## Files to touch

- `Cargo.toml` — add web-sys features: `CanvasRenderingContext2d`, `HtmlCanvasElement`, `HtmlImageElement`, `Document`
- `src/compress.rs` — NEW module: pure-Rust helpers + canvas compression
- `src/main.rs` — declare `mod compress;`
- `src/components/dashboard.rs` — wire compress button and modal

## New module: `src/compress.rs`

### Pure-Rust functions (natively testable)

```rust
/// Returns true for JPEG, PNG, WebP — compressible raster formats.
pub fn is_compressible_image(name: &str) -> bool {
    matches!(
        name.rsplit('.').next().unwrap_or("").to_lowercase().as_str(),
        "jpg" | "jpeg" | "png" | "webp"
    )
}

/// Scale (width, height) so neither dimension exceeds max_size,
/// preserving aspect ratio. Does NOT upscale.
pub fn scale_dimensions(width: u32, height: u32, max_size: u32) -> (u32, u32) {
    if width <= max_size && height <= max_size {
        return (width, height);
    }
    if width >= height {
        let h = (height as f64 * max_size as f64 / width as f64).round() as u32;
        (max_size, h.max(1))
    } else {
        let w = (width as f64 * max_size as f64 / height as f64).round() as u32;
        (w.max(1), max_size)
    }
}

/// MIME type from file extension. Defaults to "image/jpeg".
pub fn mime_type_from_name(name: &str) -> &'static str {
    match name.rsplit('.').next().unwrap_or("").to_lowercase().as_str() {
        "png" => "image/png",
        _ => "image/jpeg",
    }
}
```

### Canvas compression (WASM-only, `#[cfg(target_arch = "wasm32")]`)

```rust
pub struct CompressResult {
    pub original_width: u32,
    pub original_height: u32,
    pub new_width: u32,
    pub new_height: u32,
    pub compressed_bytes: Vec<u8>,
    pub preview_data_url: String,
}

pub async fn compress_image(bytes: &[u8], file_name: &str) -> Result<CompressResult, String>
```

Implementation steps for `compress_image`:
1. Base64-encode bytes → `data:{mime};base64,{b64}` URL
2. `HtmlImageElement::new()`, set `.src` to data URL
3. Wrap `onload` in `js_sys::Promise` → `wasm_bindgen_futures::JsFuture` to await load
4. Read `natural_width()` / `natural_height()` from image
5. `scale_dimensions(w, h, 1000)` → `(new_w, new_h)`
6. `gloo_utils::document().create_element("canvas")` → `HtmlCanvasElement`, set width/height
7. `canvas.get_context("2d")` → `CanvasRenderingContext2d`
8. `ctx.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, new_w as f64, new_h as f64)`
9. `canvas.to_data_url_with_type_and_quality(output_mime, 0.85)` → base64 data URL string
10. Strip `data:…;base64,` prefix → `BASE64_STANDARD.decode(rest)` → `Vec<u8>`
11. Return `CompressResult { original_width: w, original_height: h, new_width: new_w, new_height: new_h, compressed_bytes, preview_data_url }`

Output MIME:
- `.png` → `"image/png"` (quality param ignored by canvas for PNG)
- `.jpg` / `.jpeg` / `.webp` → `"image/jpeg"` quality 0.85

For the onload promise pattern:
```rust
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

let promise = js_sys::Promise::new(&mut |resolve, reject| {
    let img = HtmlImageElement::new().unwrap();
    let img_clone = img.clone();
    let onload = Closure::once(Box::new(move || {
        resolve.call1(&wasm_bindgen::JsValue::NULL, &img_clone).unwrap();
    }) as Box<dyn FnOnce()>);
    let onerror = Closure::once(Box::new(move || {
        reject.call1(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str("Image load failed")).unwrap();
    }) as Box<dyn FnOnce()>);
    img.set_onload(Some(onload.as_ref().unchecked_ref()));
    img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onload.forget();
    onerror.forget();
    img.set_src(&data_url);
});
let js_img = wasm_bindgen_futures::JsFuture::from(promise).await
    .map_err(|e| format!("Image load error: {:?}", e))?;
let img: HtmlImageElement = js_img.dyn_into::<HtmlImageElement>().unwrap();
```

## Dashboard changes (`src/components/dashboard.rs`)

### State (add alongside delete state)

```rust
let compress_workflow = use_state(|| Option::<CompressWorkflow>::None);
```

Where:
```rust
#[derive(Clone, PartialEq)]
enum CompressPhase {
    Downloading,
    Compressing,
    Preview {
        original_size: u64,
        original_width: u32,
        original_height: u32,
        compressed_bytes: std::rc::Rc<Vec<u8>>,
        compressed_width: u32,
        compressed_height: u32,
        preview_data_url: String,
    },
    Uploading,
    Error(String),
}

#[derive(Clone, PartialEq)]
struct CompressWorkflow {
    entry: ContentEntry,
    phase: CompressPhase,
}
```

### Callback: `on_compress_request`

```rust
let on_compress_request = {
    // clones: compress_workflow, token, active_branch_opt
    Callback::from(move |entry: ContentEntry| {
        compress_workflow.set(Some(CompressWorkflow {
            entry: entry.clone(),
            phase: CompressPhase::Downloading,
        }));
        let branch = active_branch_opt.clone().unwrap_or_else(|| DEFAULT_BRANCH.to_string());
        // spawn_local:
        //   1. client.get_file_bytes(&entry.path, &branch).await
        //   2. set phase to Compressing
        //   3. compress::compress_image(&bytes, &entry.name).await
        //   4. set phase to Preview { ... }
        //   On any Err: set phase to Error(msg)
    })
};
```

### Callback: `on_compress_cancel`
Set `compress_workflow` to `None`.

### Callback: `on_compress_confirm`
Extract compressed_bytes and entry from Preview phase, set phase to Uploading, spawn_local:
1. `client.upload_binary_file(&entry.path, &compressed_bytes, &message, Some(&entry.sha), &branch).await`
2. On Ok: `invalidate_all_caches()`, `force_refresh.set(...)`, `compress_workflow.set(None)`
3. On Err: set phase to `Error(msg)`

### Compress button in `render_entry`

Add `on_compress: Callback<ContentEntry>` parameter to `render_entry`.

```rust
let compress_btn = if crate::compress::is_compressible_image(&entry.name) && is_authenticated {
    let entry_clone = entry.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
        on_compress.emit(entry_clone.clone());
    });
    html! { <button class="entry-compress-btn" onclick={onclick}>{"Compress"}</button> }
} else {
    html! {}
};
```

Place `{compress_btn}` before `{delete_btn}` in the entry HTML.

Also update the global search results section in `dashboard()` to show a compress button for authenticated image entries, mirroring the delete button pattern there.

### Compress modal (after delete confirm modal block)

```rust
if let Some(ref wf) = *compress_workflow {
    <div class="modal-overlay" onclick={on_compress_cancel.clone()}>
        <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
            match &wf.phase {
                CompressPhase::Downloading => <p>{"Downloading…"}</p>
                CompressPhase::Compressing => <p>{"Compressing…"}</p>
                CompressPhase::Uploading   => <p>{"Uploading…"}</p>
                CompressPhase::Error(msg)  => {
                    <p class="error">{msg}</p>
                    <button onclick={on_compress_cancel}>{"Close"}</button>
                }
                CompressPhase::Preview { original_size, original_width, original_height,
                                         compressed_bytes, compressed_width, compressed_height,
                                         preview_data_url } => {
                    // header: file name
                    // table: Original size | New size | Old dims | New dims
                    // warning if compressed_bytes.len() as u64 >= *original_size
                    // <img src={preview_data_url} class="compress-preview" />
                    // Confirm and Cancel buttons
                }
            }
        </div>
    </div>
}
```

## Cargo.toml

Add to the web-sys features list:
```
"CanvasRenderingContext2d",
"HtmlCanvasElement",
"HtmlImageElement",
"Document",
```

## Unit tests (`src/compress.rs`, `#[cfg(test)]` block, native)

```rust
#[test] fn scale_no_change_when_within_limit()     // both ≤ max → unchanged
#[test] fn scale_wide_image()                      // 2000×1000, max=1000 → 1000×500
#[test] fn scale_tall_image()                      // 1000×2000, max=1000 → 500×1000
#[test] fn scale_square_image()                    // 2000×2000, max=1000 → 1000×1000
#[test] fn scale_does_not_upscale()               // 400×300, max=1000 → 400×300
#[test] fn scale_preserves_ratio()                // ratio within 0.5% of original
#[test] fn is_compressible_image_jpg()
#[test] fn is_compressible_image_jpeg()
#[test] fn is_compressible_image_png()
#[test] fn is_compressible_image_webp()
#[test] fn is_compressible_image_svg_false()
#[test] fn is_compressible_image_gif_false()
#[test] fn is_compressible_image_mp4_false()
#[test] fn mime_type_from_name_png()
#[test] fn mime_type_from_name_jpeg()
#[test] fn mime_type_from_name_jpg()
#[test] fn mime_type_from_name_webp_returns_jpeg()
```

## WASM tests (`tests/wasm.rs`)

Add:
```rust
#[wasm_bindgen_test]
async fn compress_tiny_jpeg_returns_valid_output() {
    // Minimal valid 1×1 white JPEG (known bytes)
    let jpeg_bytes: &[u8] = &[
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43,
        0x00, 0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09,
        0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12,
        0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20,
        0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29,
        0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32,
        0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01,
        0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00, 0x1F, 0x00, 0x00,
        0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A, 0x0B, 0xFF, 0xC4, 0x00, 0xB5, 0x10, 0x00, 0x02, 0x01, 0x03,
        0x03, 0x02, 0x04, 0x03, 0x05, 0x05, 0x04, 0x04, 0x00, 0x00, 0x01, 0x7D,
        0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06,
        0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xA1, 0x08,
        0x23, 0x42, 0xB1, 0xC1, 0x15, 0x52, 0xD1, 0xF0, 0x24, 0x33, 0x62, 0x72,
        0x82, 0x09, 0x0A, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x25, 0x26, 0x27, 0x28,
        0x29, 0x2A, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x43, 0x44, 0x45,
        0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x73, 0x74, 0x75,
        0x76, 0x77, 0x78, 0x79, 0x7A, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
        0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3,
        0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6,
        0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9,
        0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE1, 0xE2,
        0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF1, 0xF2, 0xF3, 0xF4,
        0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01,
        0x00, 0x00, 0x3F, 0x00, 0xFB, 0xD2, 0x8A, 0x28, 0x03, 0xFF, 0xD9,
    ];
    let result = blog_editor::compress::compress_image(jpeg_bytes, "test.jpg")
        .await
        .expect("compress should succeed");
    assert!(result.compressed_bytes.len() > 0);
    assert_eq!(result.original_width, 1);
    assert_eq!(result.original_height, 1);
    assert_eq!(result.new_width, 1);   // 1×1 is below max, not upscaled
    assert_eq!(result.new_height, 1);
    assert!(result.preview_data_url.starts_with("data:image/"));
}
```

Note: the WASM test requires `pub mod compress` and `pub use compress::compress_image` to be accessible from tests. Ensure `lib.rs` or `main.rs` exposes the module appropriately for test access. Since this is a `[[bin]]` crate with no `lib.rs`, the WASM test in `tests/wasm.rs` may need to be structured as an integration test using `wasm_bindgen_test`. The canvas compress function can be tested via a helper re-export or by making `compress.rs` items pub. See existing wasm test setup — currently tests don't call any crate functions.

If exposing crate internals is not straightforward due to the bin-crate structure, test the compress_image function by calling it from within the test file directly (since `wasm_bindgen_test` crates can include mod declarations). Alternatively, inline the compress function in the test. Check CLAUDE.md for testing conventions.

## Validation

After implementing:
```sh
cargo fmt
cargo check --target wasm32-unknown-unknown
cargo clippy
cargo test
wasm-pack test --headless --chrome --chromedriver $(which chromedriver)
```

All must pass with zero `#[allow(dead_code)]`.

## Out of scope

- Dithering (Canvas API does not support it; the resize alone is the primary size reducer)
- GIF compression (animated GIFs require special handling)
- SVG optimization
- Compression progress bar
