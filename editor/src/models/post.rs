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

/// Render markdown content to HTML, stripping any TOML frontmatter first.
/// Uses GFM (GitHub Flavored Markdown) options for tables, strikethrough, etc.
pub fn render_markdown(content: &str) -> String {
    let body = strip_frontmatter(content);
    match markdown::to_html_with_options(body, &markdown::Options::gfm()) {
        Ok(html) => html,
        Err(_) => markdown::to_html(body),
    }
}
