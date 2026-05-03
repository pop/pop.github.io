/// Returns true for JPEG, PNG, WebP — compressible raster formats.
pub fn is_compressible_image(name: &str) -> bool {
    matches!(
        name.rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase()
            .as_str(),
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
    match name
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "png" => "image/png",
        _ => "image/jpeg",
    }
}

pub use platform_impl::*;

#[cfg(target_arch = "wasm32")]
mod platform_impl {
    use super::{mime_type_from_name, scale_dimensions};
    use base64::prelude::*;
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

    pub struct CompressResult {
        pub original_width: u32,
        pub original_height: u32,
        pub new_width: u32,
        pub new_height: u32,
        pub compressed_bytes: Vec<u8>,
        pub preview_data_url: String,
    }

    pub async fn compress_image(bytes: &[u8], file_name: &str) -> Result<CompressResult, String> {
        let mime = mime_type_from_name(file_name);
        let b64 = BASE64_STANDARD.encode(bytes);
        let data_url = format!("data:{mime};base64,{b64}");

        // Load image via Promise
        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            let img = HtmlImageElement::new().unwrap();
            let img_clone = img.clone();
            let onload = Closure::once(Box::new(move || {
                resolve.call1(&JsValue::NULL, &img_clone).unwrap();
            }) as Box<dyn FnOnce()>);
            let onerror = Closure::once(Box::new(move || {
                reject
                    .call1(&JsValue::NULL, &JsValue::from_str("Image load failed"))
                    .unwrap();
            }) as Box<dyn FnOnce()>);
            img.set_onload(Some(onload.as_ref().unchecked_ref()));
            img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            onload.forget();
            onerror.forget();
            img.set_src(&data_url);
        });

        let js_img = JsFuture::from(promise)
            .await
            .map_err(|e| format!("Image load error: {e:?}"))?;
        let img: HtmlImageElement = js_img
            .dyn_into::<HtmlImageElement>()
            .map_err(|e| format!("Not an image element: {e:?}"))?;

        let orig_w = img.natural_width();
        let orig_h = img.natural_height();
        let (new_w, new_h) = scale_dimensions(orig_w, orig_h, 1000);

        let canvas: HtmlCanvasElement = gloo_utils::document()
            .create_element("canvas")
            .map_err(|e| format!("{e:?}"))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|e| format!("{e:?}"))?;

        canvas.set_width(new_w);
        canvas.set_height(new_h);

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .map_err(|e| format!("{e:?}"))?
            .ok_or("No 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|e| format!("{e:?}"))?;

        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &img,
            0.0,
            0.0,
            new_w as f64,
            new_h as f64,
        )
        .map_err(|e| format!("{e:?}"))?;

        let output_mime = if file_name.rsplit('.').next().unwrap_or("").to_lowercase() == "png" {
            "image/png"
        } else {
            "image/jpeg"
        };

        let quality = JsValue::from_f64(0.85);
        let preview_data_url = canvas
            .to_data_url_with_type_and_encoder_options(output_mime, &quality)
            .map_err(|e| format!("{e:?}"))?;

        // Strip data URL prefix and decode to bytes
        let prefix = format!("data:{output_mime};base64,");
        let rest = preview_data_url
            .strip_prefix(&prefix)
            .ok_or_else(|| format!("Unexpected data URL format: missing prefix {prefix}"))?;

        let compressed_bytes = BASE64_STANDARD
            .decode(rest)
            .map_err(|e| format!("Base64 decode error: {e}"))?;

        Ok(CompressResult {
            original_width: orig_w,
            original_height: orig_h,
            new_width: new_w,
            new_height: new_h,
            compressed_bytes,
            preview_data_url,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform_impl {
    pub struct CompressResult {
        pub original_width: u32,
        pub original_height: u32,
        pub new_width: u32,
        pub new_height: u32,
        pub compressed_bytes: Vec<u8>,
        pub preview_data_url: String,
    }

    pub async fn compress_image(_bytes: &[u8], _file_name: &str) -> Result<CompressResult, String> {
        Err("compress_image is only available in WASM".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_no_change_when_within_limit() {
        assert_eq!(scale_dimensions(500, 300, 1000), (500, 300));
    }

    #[test]
    fn scale_wide_image() {
        assert_eq!(scale_dimensions(2000, 1000, 1000), (1000, 500));
    }

    #[test]
    fn scale_tall_image() {
        assert_eq!(scale_dimensions(1000, 2000, 1000), (500, 1000));
    }

    #[test]
    fn scale_square_image() {
        assert_eq!(scale_dimensions(2000, 2000, 1000), (1000, 1000));
    }

    #[test]
    fn scale_does_not_upscale() {
        assert_eq!(scale_dimensions(400, 300, 1000), (400, 300));
    }

    #[test]
    fn scale_preserves_ratio() {
        let (w, h) = scale_dimensions(3000, 2000, 1000);
        assert_eq!(w, 1000);
        assert_eq!(h, 667);
        let orig_ratio = 3000.0 / 2000.0_f64;
        let new_ratio = w as f64 / h as f64;
        let diff = (orig_ratio - new_ratio).abs() / orig_ratio;
        assert!(diff < 0.005, "ratio diff {diff} >= 0.5%");
    }

    #[test]
    fn is_compressible_image_jpg() {
        assert!(is_compressible_image("photo.jpg"));
    }

    #[test]
    fn is_compressible_image_jpeg() {
        assert!(is_compressible_image("photo.jpeg"));
    }

    #[test]
    fn is_compressible_image_png() {
        assert!(is_compressible_image("image.png"));
    }

    #[test]
    fn is_compressible_image_webp() {
        assert!(is_compressible_image("image.webp"));
    }

    #[test]
    fn is_compressible_image_svg_false() {
        assert!(!is_compressible_image("icon.svg"));
    }

    #[test]
    fn is_compressible_image_gif_false() {
        assert!(!is_compressible_image("anim.gif"));
    }

    #[test]
    fn is_compressible_image_mp4_false() {
        assert!(!is_compressible_image("video.mp4"));
    }

    #[test]
    fn mime_type_from_name_png() {
        assert_eq!(mime_type_from_name("image.png"), "image/png");
    }

    #[test]
    fn mime_type_from_name_jpeg() {
        assert_eq!(mime_type_from_name("photo.jpeg"), "image/jpeg");
    }

    #[test]
    fn mime_type_from_name_jpg() {
        assert_eq!(mime_type_from_name("photo.jpg"), "image/jpeg");
    }

    #[test]
    fn mime_type_from_name_webp_returns_jpeg() {
        assert_eq!(mime_type_from_name("image.webp"), "image/jpeg");
    }
}
