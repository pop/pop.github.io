use base64::prelude::*;
use gloo_net::http::Request;
use serde_json::json;

use crate::models::github::{ContentEntry, FileContent, GitRef};

const OWNER: &str = "pop";
const REPO: &str = "pop.github.io";
const API_BASE: &str = "https://api.github.com";

pub struct GitHubClient {
    pub token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    // ── Directory & file reading ─────────────────────────────────

    /// List the contents of a directory in the repo (reads from the `source` branch).
    pub async fn list_contents(&self, path: &str) -> Result<Vec<ContentEntry>, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref=source");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err(format!("Path not found: {path}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    /// Read a single file from the repo on a specific branch.
    pub async fn get_file(&self, path: &str, branch: &str) -> Result<FileContent, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref={branch}");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err(format!("File not found: {path}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    // ── Branch operations ────────────────────────────────────────

    /// Get the HEAD SHA of a branch.
    pub async fn get_branch_sha(&self, branch: &str) -> Result<String, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/ref/heads/{branch}");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => {
                let git_ref: GitRef = resp.json().await.map_err(|e| e.to_string())?;
                Ok(git_ref.object.sha)
            }
            404 => Err(format!("Branch not found: {branch}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    /// Create a new branch pointing at the given SHA.
    pub async fn create_branch(
        &self,
        branch_name: &str,
        from_sha: &str,
    ) -> Result<(), String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/refs");
        let body = json!({
            "ref": format!("refs/heads/{branch_name}"),
            "sha": from_sha,
        });

        let resp = Request::post(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            201 => Ok(()),
            422 => Err("Branch already exists".into()),
            status => Err(format!("Failed to create branch: {status}")),
        }
    }

    /// Delete a branch.
    pub async fn delete_branch(&self, branch_name: &str) -> Result<(), String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/refs/heads/{branch_name}");
        let resp = Request::delete(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            204 => Ok(()),
            status => Err(format!("Failed to delete branch: {status}")),
        }
    }

    // ── File mutations ───────────────────────────────────────────

    /// Create or update a file on a branch. Returns the new file SHA.
    ///
    /// Pass `sha = None` for new files, `sha = Some(...)` for updates.
    pub async fn create_or_update_file(
        &self,
        path: &str,
        content: &str,
        message: &str,
        sha: Option<&str>,
        branch: &str,
    ) -> Result<String, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}");
        let encoded = BASE64_STANDARD.encode(content.as_bytes());

        let mut body = json!({
            "message": message,
            "content": encoded,
            "branch": branch,
        });

        if let Some(sha) = sha {
            body.as_object_mut().unwrap().insert("sha".into(), json!(sha));
        }

        let resp = Request::put(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            200 | 201 => {
                let result: serde_json::Value =
                    resp.json().await.map_err(|e| e.to_string())?;
                let sha = result["content"]["sha"]
                    .as_str()
                    .ok_or("Missing SHA in response")?
                    .to_string();
                Ok(sha)
            }
            409 => Err("Conflict \u{2014} file was modified elsewhere".into()),
            status => Err(format!("Failed to save file: {status}")),
        }
    }

    /// Delete a file on a branch.
    pub async fn delete_file(
        &self,
        path: &str,
        sha: &str,
        message: &str,
        branch: &str,
    ) -> Result<(), String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}");
        let body = json!({
            "message": message,
            "sha": sha,
            "branch": branch,
        });

        let resp = Request::delete(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            200 => Ok(()),
            status => Err(format!("Failed to delete file: {status}")),
        }
    }

    // ── Binary uploads ───────────────────────────────────────────

    /// Upload a binary file (e.g., image) to a branch. Returns the new file SHA.
    pub async fn upload_binary_file(
        &self,
        path: &str,
        data: &[u8],
        message: &str,
        branch: &str,
    ) -> Result<String, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}");
        let encoded = BASE64_STANDARD.encode(data);

        let body = json!({
            "message": message,
            "content": encoded,
            "branch": branch,
        });

        let resp = Request::put(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            200 | 201 => {
                let result: serde_json::Value =
                    resp.json().await.map_err(|e| e.to_string())?;
                let sha = result["content"]["sha"]
                    .as_str()
                    .ok_or("Missing SHA in response")?
                    .to_string();
                Ok(sha)
            }
            409 => Err("Conflict \u{2014} image already exists at this path".into()),
            status => Err(format!("Failed to upload image: {status}")),
        }
    }

    // ── Helpers ──────────────────────────────────────────────────

    async fn get(&self, url: &str) -> Result<gloo_net::http::Response, String> {
        Request::get(url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .send()
            .await
            .map_err(|e| e.to_string())
    }
}

/// Decode base64 file content from the GitHub API (which includes newlines).
pub fn decode_github_content(encoded: &str) -> String {
    let cleaned: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
    match BASE64_STANDARD.decode(cleaned.as_bytes()) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}
