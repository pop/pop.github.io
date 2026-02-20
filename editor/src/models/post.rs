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
            let replacement = replacements.get(src_value).map(|s| s.as_str()).unwrap_or(src_value);
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
