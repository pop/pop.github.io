---
# editor-22tt
title: Add get_file_bytes to GitHubClient
status: completed
type: task
priority: high
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

Add pub async fn get_file_bytes(&self, path: &str, branch: &str) -> Result<Vec<u8>, String> to src/services/github.rs. Mirrors get_file_rest pattern but returns raw bytes decoded from base64 instead of String. Uses REST Contents API: GET /repos/pop/pop.github.io/contents/{path}?ref={branch}. Parses FileContent JSON, strips whitespace from base64, decodes with BASE64_STANDARD.decode.
