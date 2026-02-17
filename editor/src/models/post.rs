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
