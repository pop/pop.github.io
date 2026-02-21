/// Extract the post slug from a file path.
///
/// - `content/blog/my-post.md`  → `"my-post"`
/// - `content/blog/my-post/index.md` → `"my-post"`
pub fn slug_from_path(path: &str) -> String {
    let parts: Vec<&str> = path.trim_end_matches('/').split('/').collect();
    match parts.last() {
        Some(&"index.md") | Some(&"_index.md") => parts
            .get(parts.len().wrapping_sub(2))
            .unwrap_or(&"untitled")
            .to_string(),
        Some(last) => last.trim_end_matches(".md").to_string(),
        None => "untitled".to_string(),
    }
}

/// Convert a kebab-case slug into a title-cased display string.
pub fn title_from_slug(slug: &str) -> String {
    let title = slug.replace('-', " ");
    let mut chars = title.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

/// Sanitize a user-supplied filename: spaces → hyphens, keep alphanumeric / `-` / `_` / `.`.
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c == ' ' { '-' } else { c })
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect::<String>()
        .to_lowercase()
}

/// Convert a character index into the corresponding byte offset within `s`.
pub fn char_pos_to_byte_offset(s: &str, char_pos: usize) -> usize {
    s.char_indices()
        .nth(char_pos)
        .map_or(s.len(), |(byte_idx, _)| byte_idx)
}

/// Format a byte count as a human-readable string (B / KB / MB).
pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slug_from_path_plain_file() {
        assert_eq!(slug_from_path("content/blog/my-post.md"), "my-post");
    }

    #[test]
    fn test_slug_from_path_index() {
        assert_eq!(slug_from_path("content/blog/my-post/index.md"), "my-post");
    }

    #[test]
    fn test_slug_from_path_underscore_index() {
        assert_eq!(slug_from_path("content/_index.md"), "content");
    }

    #[test]
    fn test_title_from_slug() {
        assert_eq!(title_from_slug("my-post"), "My post");
        assert_eq!(title_from_slug("hello-world"), "Hello world");
        assert_eq!(title_from_slug(""), "");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Hello World!"), "hello-world");
        assert_eq!(sanitize_filename("my_post.md"), "my_post.md");
        assert_eq!(sanitize_filename("foo bar baz"), "foo-bar-baz");
    }

    #[test]
    fn test_char_pos_to_byte_offset_ascii() {
        let s = "hello";
        assert_eq!(char_pos_to_byte_offset(s, 0), 0);
        assert_eq!(char_pos_to_byte_offset(s, 3), 3);
        assert_eq!(char_pos_to_byte_offset(s, 10), 5); // past end → s.len()
    }

    #[test]
    fn test_char_pos_to_byte_offset_multibyte() {
        let s = "hé"; // 'é' is 2 bytes
        assert_eq!(char_pos_to_byte_offset(s, 0), 0);
        assert_eq!(char_pos_to_byte_offset(s, 1), 1);
        assert_eq!(char_pos_to_byte_offset(s, 2), 3); // past end
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
    }
}
