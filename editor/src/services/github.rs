use gloo_net::http::Request;

use crate::models::github::{ContentEntry, FileContent};

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

    /// List the contents of a directory in the repo (reads from the `source` branch).
    pub async fn list_contents(&self, path: &str) -> Result<Vec<ContentEntry>, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref=source");
        let resp = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized — check your token".into()),
            404 => Err(format!("Path not found: {path}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    /// Read the content of a single file from the repo.
    pub async fn get_file(&self, path: &str) -> Result<FileContent, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref=source");
        let resp = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized — check your token".into()),
            404 => Err(format!("File not found: {path}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }
}
