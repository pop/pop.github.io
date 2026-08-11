use std::collections::HashMap;

use base64::prelude::*;

/// Strip TOML frontmatter (delimited by `+++`) from content,
/// returning just the markdown body.
pub fn strip_frontmatter(content: &str) -> &str {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("+++") {
        return content;
    }
    // Find the closing +++ after the opening one
    if let Some(end) = trimmed[3..].find("\n+++") {
        let after = &trimmed[3 + end + 4..]; // skip past "\n+++"
        after.strip_prefix('\n').unwrap_or(after)
    } else {
        content
    }
}

/// Extract the raw TOML text between `+++` delimiters, if present.
pub fn extract_frontmatter(content: &str) -> Option<&str> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("+++") {
        return None;
    }
    let after_open = &trimmed[3..];
    let after_open = after_open.strip_prefix('\n').unwrap_or(after_open);
    if let Some(end) = after_open.find("\n+++") {
        Some(&after_open[..end])
    } else {
        None
    }
}

/// Parse frontmatter into key-value pairs for display.
/// Returns an empty vec on parse failure or missing frontmatter.
pub fn parse_frontmatter(content: &str) -> Vec<(String, String)> {
    let raw = match extract_frontmatter(content) {
        Some(s) => s,
        None => return Vec::new(),
    };
    let table: toml::Table = match toml::from_str(raw) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    flatten_toml("", &table, &mut out);
    out
}

fn format_toml_value(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Array(arr) => arr
            .iter()
            .map(format_toml_value)
            .collect::<Vec<_>>()
            .join(", "),
        toml::Value::Datetime(dt) => dt.to_string(),
        toml::Value::Table(_) => String::new(), // handled by flattening
    }
}

fn flatten_toml(prefix: &str, table: &toml::Table, out: &mut Vec<(String, String)>) {
    for (key, value) in table {
        let full_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        if let toml::Value::Table(inner) = value {
            flatten_toml(&full_key, inner, out);
        } else {
            out.push((full_key, format_toml_value(value)));
        }
    }
}

/// Render markdown content to HTML, stripping any TOML frontmatter first.
/// Uses GFM (GitHub Flavored Markdown) options for tables, strikethrough, etc.
pub fn render_markdown(content: &str) -> String {
    let body = strip_frontmatter(content);
    match markdown::to_html_with_options(body, &markdown::Options::gfm()) {
        Ok(html) => html,
        Err(_) => markdown::to_html(body),
    }
}

/// Extract plain text from rendered HTML by stripping tags and decoding a
/// small set of common entities. Whitespace runs collapse to a single space.
fn html_to_plain_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    let decoded = out
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Count words and characters in the *rendered prose* of a post — frontmatter
/// and markdown syntax are excluded. Characters include spaces between words.
pub fn count_prose(content: &str) -> (usize, usize) {
    let plain = html_to_plain_text(&render_markdown(content));
    let words = if plain.is_empty() {
        0
    } else {
        plain.split_whitespace().count()
    };
    let chars = plain.chars().count();
    (words, chars)
}

/// Return the parent directory of a repo file path (everything before the last `/`).
/// Returns `""` for top-level files with no directory component.
pub fn post_dir(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/// Scan rendered HTML for `<img` tags and return all relative `src` values.
///
/// A src is considered relative if it does not start with `http`, `//`, `/`,
/// or `data:`.
pub fn extract_relative_image_srcs(html: &str) -> Vec<String> {
    let mut srcs = Vec::new();
    let mut rest = html;
    while let Some(img_pos) = rest.find("<img") {
        rest = &rest[img_pos + 4..];
        // Find the end of this tag
        let tag_end = rest.find('>').unwrap_or(rest.len());
        let tag = &rest[..tag_end];
        // Look for src= within the tag
        if let Some(src_pos) = tag.find("src=") {
            let after_src = &tag[src_pos + 4..];
            let src_value = if after_src.starts_with('"') {
                after_src[1..].split('"').next().unwrap_or("")
            } else if after_src.starts_with('\'') {
                after_src[1..].split('\'').next().unwrap_or("")
            } else {
                after_src.split_whitespace().next().unwrap_or("")
            };
            if !src_value.is_empty()
                && !src_value.starts_with("http")
                && !src_value.starts_with("//")
                && !src_value.starts_with('/')
                && !src_value.starts_with("data:")
            {
                srcs.push(src_value.to_string());
            }
        }
    }
    srcs
}

/// Replace `src` attribute values in `<img` tags according to the given map.
pub fn replace_image_srcs(html: &str, replacements: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(img_pos) = rest.find("<img") {
        result.push_str(&rest[..img_pos + 4]);
        rest = &rest[img_pos + 4..];
        let tag_end = rest.find('>').unwrap_or(rest.len());
        let tag = &rest[..tag_end];
        if let Some(src_pos) = tag.find("src=") {
            let (before_src, from_src) = tag.split_at(src_pos + 4);
            result.push_str(before_src);
            let (quote, after_quote) = if from_src.starts_with('"') {
                ('"', &from_src[1..])
            } else if from_src.starts_with('\'') {
                ('\'', &from_src[1..])
            } else {
                ('\0', from_src)
            };
            let (src_value, after_value) = if quote != '\0' {
                let end = after_quote.find(quote).unwrap_or(after_quote.len());
                (&after_quote[..end], &after_quote[end..])
            } else {
                let end = after_quote
                    .find(|c: char| c.is_whitespace() || c == '>')
                    .unwrap_or(after_quote.len());
                (&after_quote[..end], &after_quote[end..])
            };
            let replacement = replacements
                .get(src_value)
                .map(|s| s.as_str())
                .unwrap_or(src_value);
            if quote != '\0' {
                result.push(quote);
                result.push_str(replacement);
                result.push_str(after_value);
            } else {
                result.push_str(replacement);
                result.push_str(after_value);
            }
        } else {
            result.push_str(tag);
        }
        rest = &rest[tag_end..];
    }
    result.push_str(rest);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── strip_frontmatter ──────────────────────────────────────────────────

    #[test]
    fn strip_frontmatter_with_frontmatter() {
        let content = "+++\ntitle = \"Hello\"\n+++\n# Body";
        assert_eq!(strip_frontmatter(content), "# Body");
    }

    #[test]
    fn strip_frontmatter_no_frontmatter() {
        let content = "# No frontmatter here";
        assert_eq!(strip_frontmatter(content), content);
    }

    #[test]
    fn strip_frontmatter_empty_string() {
        assert_eq!(strip_frontmatter(""), "");
    }

    #[test]
    fn strip_frontmatter_unclosed_delimiter() {
        // No closing +++, so the whole string is returned unchanged
        let content = "+++\ntitle = \"Hello\"\n# Body";
        assert_eq!(strip_frontmatter(content), content);
    }

    #[test]
    fn strip_frontmatter_leading_whitespace() {
        let content = "\n+++\ntitle = \"x\"\n+++\nbody";
        assert_eq!(strip_frontmatter(content), "body");
    }

    #[test]
    fn strip_frontmatter_no_newline_after_closing() {
        let content = "+++\ntitle = \"x\"\n+++";
        // After stripping, the body is empty
        assert_eq!(strip_frontmatter(content), "");
    }

    // ── render_markdown ────────────────────────────────────────────────────

    #[test]
    fn render_markdown_basic() {
        let html = render_markdown("# Hello");
        assert!(html.contains("<h1>"), "expected h1 tag, got: {html}");
        assert!(html.contains("Hello"));
    }

    #[test]
    fn render_markdown_strips_frontmatter() {
        let content = "+++\ntitle = \"x\"\n+++\n**bold**";
        let html = render_markdown(content);
        assert!(html.contains("<strong>bold</strong>"), "got: {html}");
        assert!(
            !html.contains("title"),
            "frontmatter leaked into output: {html}"
        );
    }

    #[test]
    fn render_markdown_gfm_strikethrough() {
        let html = render_markdown("~~struck~~");
        assert!(html.contains("<del>struck</del>"), "got: {html}");
    }

    // ── count_prose ────────────────────────────────────────────────────────

    #[test]
    fn count_prose_empty() {
        assert_eq!(count_prose(""), (0, 0));
    }

    #[test]
    fn count_prose_ignores_frontmatter() {
        let content = "+++\ntitle = \"Hello there world\"\n+++\nhi";
        assert_eq!(count_prose(content), (1, 2));
    }

    #[test]
    fn count_prose_strips_markdown_syntax() {
        // "one two three" — headings, bold, links contribute their visible
        // text only, not the syntax characters.
        let content = "# one\n\n**two** [three](https://example.com)";
        assert_eq!(count_prose(content), (3, 13));
    }

    #[test]
    fn count_prose_collapses_whitespace_between_blocks() {
        let content = "one two\n\nthree four";
        assert_eq!(count_prose(content), (4, 18));
    }

    // ── post_dir ───────────────────────────────────────────────────────────

    #[test]
    fn post_dir_nested() {
        assert_eq!(post_dir("content/blog/my-post.md"), "content/blog");
    }

    #[test]
    fn post_dir_top_level() {
        assert_eq!(post_dir("README.md"), "");
    }

    // ── extract_relative_image_srcs ───────────────────────────────────────

    #[test]
    fn extract_relative_srcs_finds_relative() {
        let html = r#"<img src="photo.png" alt="x">"#;
        assert_eq!(extract_relative_image_srcs(html), vec!["photo.png"]);
    }

    #[test]
    fn extract_relative_srcs_ignores_absolute() {
        let html = r#"<img src="https://example.com/photo.png">"#;
        assert!(extract_relative_image_srcs(html).is_empty());
    }

    #[test]
    fn extract_relative_srcs_ignores_data_url() {
        let html = r#"<img src="data:image/png;base64,abc">"#;
        assert!(extract_relative_image_srcs(html).is_empty());
    }

    #[test]
    fn extract_relative_srcs_ignores_root_relative() {
        let html = r#"<img src="/images/photo.png">"#;
        assert!(extract_relative_image_srcs(html).is_empty());
    }

    #[test]
    fn extract_relative_srcs_multiple() {
        let html = r#"<img src="a.png"><img src="https://x.com/b.png"><img src="c.jpg">"#;
        assert_eq!(extract_relative_image_srcs(html), vec!["a.png", "c.jpg"]);
    }

    // ── replace_image_srcs ────────────────────────────────────────────────

    #[test]
    fn replace_image_srcs_substitutes_matching() {
        let html = r#"<img src="photo.png" alt="x">"#;
        let mut map = HashMap::new();
        map.insert(
            "photo.png".to_string(),
            "data:image/png;base64,ABC".to_string(),
        );
        let result = replace_image_srcs(html, &map);
        assert!(
            result.contains("data:image/png;base64,ABC"),
            "got: {result}"
        );
        assert!(!result.contains("photo.png"), "got: {result}");
    }

    #[test]
    fn replace_image_srcs_leaves_unmatched_alone() {
        let html = r#"<img src="other.png">"#;
        let map = HashMap::new();
        let result = replace_image_srcs(html, &map);
        assert_eq!(result, html);
    }

    #[test]
    fn replace_image_srcs_no_img_tags() {
        let html = "<p>No images here</p>";
        let map = HashMap::new();
        assert_eq!(replace_image_srcs(html, &map), html);
    }

    // ── mime_type_for ─────────────────────────────────────────────────────

    #[test]
    fn mime_type_for_known_extensions() {
        assert_eq!(mime_type_for("photo.png"), "image/png");
        assert_eq!(mime_type_for("photo.jpg"), "image/jpeg");
        assert_eq!(mime_type_for("photo.jpeg"), "image/jpeg");
        assert_eq!(mime_type_for("anim.gif"), "image/gif");
        assert_eq!(mime_type_for("img.webp"), "image/webp");
        assert_eq!(mime_type_for("icon.svg"), "image/svg+xml");
    }

    #[test]
    fn mime_type_for_unknown_falls_back() {
        assert_eq!(mime_type_for("file.txt"), "application/octet-stream");
        assert_eq!(mime_type_for("noext"), "application/octet-stream");
    }

    // ── bytes_to_data_url ─────────────────────────────────────────────────

    #[test]
    fn bytes_to_data_url_produces_valid_data_url() {
        let bytes = b"hello";
        let url = bytes_to_data_url(bytes, "image.png");
        assert!(url.starts_with("data:image/png;base64,"), "got: {url}");
        // "hello" in base64 is "aGVsbG8="
        assert!(url.ends_with("aGVsbG8="), "got: {url}");
    }

    #[test]
    fn bytes_to_data_url_uses_correct_mime() {
        let url = bytes_to_data_url(b"x", "photo.jpg");
        assert!(url.starts_with("data:image/jpeg;base64,"), "got: {url}");
    }
}

/// Map a file path's extension to a MIME type string.
pub fn mime_type_for(path: &str) -> &'static str {
    let ext = path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

/// Encode raw bytes as a `data:` URL suitable for use in an `<img src>`.
pub fn bytes_to_data_url(bytes: &[u8], path: &str) -> String {
    let mime = mime_type_for(path);
    let encoded = BASE64_STANDARD.encode(bytes);
    format!("data:{mime};base64,{encoded}")
}
