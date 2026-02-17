use base64::prelude::*;
use gloo_net::http::Request;
use serde_json::json;

use crate::models::github::{
    CheckRunsResponse, CompareResponse, ContentEntry, FileContent, GitRef, TreeResponse,
};

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

    /// List the contents of a directory in the repo.
    ///
    /// Falls back to the Git Trees API if the Contents API returns 1000 entries
    /// (its hard cap), which indicates possible truncation.
    pub async fn list_contents(
        &self,
        path: &str,
        branch: Option<&str>,
    ) -> Result<Vec<ContentEntry>, String> {
        let git_ref = branch.unwrap_or("source");
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref={git_ref}");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => {
                let entries: Vec<ContentEntry> =
                    resp.json().await.map_err(|e| e.to_string())?;
                if entries.len() >= 1000 {
                    self.list_contents_via_tree(path).await
                } else {
                    Ok(entries)
                }
            }
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err(format!("Path not found: {path}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    /// Fallback directory listing using the Git Trees API (no 1000-entry cap).
    async fn list_contents_via_tree(&self, path: &str) -> Result<Vec<ContentEntry>, String> {
        let sha = self.get_branch_sha("source").await?;
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/trees/{sha}?recursive=1");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => {
                let tree: TreeResponse = resp.json().await.map_err(|e| e.to_string())?;
                let prefix = format!("{path}/");
                let entries = tree
                    .tree
                    .iter()
                    .filter_map(|te| {
                        let relative = te.path.strip_prefix(&prefix)?;
                        if relative.contains('/') {
                            return None; // not a direct child
                        }
                        Some(ContentEntry {
                            name: relative.to_string(),
                            path: te.path.clone(),
                            entry_type: if te.entry_type == "tree" {
                                "dir"
                            } else {
                                "file"
                            }
                            .to_string(),
                            sha: te.sha.clone(),
                            size: te.size.unwrap_or(0),
                            download_url: None,
                        })
                    })
                    .collect();
                Ok(entries)
            }
            401 => Err("Unauthorized \u{2014} check your token".into()),
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
            401 => Err("Unauthorized \u{2014} check your token".into()),
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
            401 => Err("Unauthorized \u{2014} check your token".into()),
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
            401 => Err("Unauthorized \u{2014} check your token".into()),
            status => Err(format!("Failed to delete branch: {status}")),
        }
    }

    // ── Merge operations ─────────────────────────────────────────

    /// Merge a head branch into a base branch.
    pub async fn merge_branch(
        &self,
        head_branch: &str,
        base_branch: &str,
        commit_message: &str,
    ) -> Result<(), String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/merges");
        let body = json!({
            "base": base_branch,
            "head": head_branch,
            "commit_message": commit_message,
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
            201 | 204 => Ok(()),
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err("Branch not found".into()),
            409 => Err("Merge conflict \u{2014} resolve manually on GitHub".into()),
            status => Err(format!("Failed to merge: {status}")),
        }
    }

    // ── Compare operations ─────────────────────────────────────

    /// Compare two branches, returning the list of changed files and patches.
    pub async fn compare_branches(
        &self,
        base: &str,
        head: &str,
    ) -> Result<CompareResponse, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/compare/{base}...{head}");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err("Branch not found".into()),
            status => Err(format!("GitHub API error: {status}")),
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
            401 => Err("Unauthorized \u{2014} check your token".into()),
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
            401 => Err("Unauthorized \u{2014} check your token".into()),
            status => Err(format!("Failed to delete file: {status}")),
        }
    }

    // ── Binary uploads ───────────────────────────────────────────

    /// Upload a binary file (e.g., image) to a branch. Returns the new file SHA.
    ///
    /// Pass `sha = None` for new files, `sha = Some(...)` to overwrite existing.
    pub async fn upload_binary_file(
        &self,
        path: &str,
        data: &[u8],
        message: &str,
        sha: Option<&str>,
        branch: &str,
    ) -> Result<String, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}");
        let encoded = BASE64_STANDARD.encode(data);

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
            401 => Err("Unauthorized \u{2014} check your token".into()),
            409 => Err("Conflict \u{2014} image already exists at this path".into()),
            status => Err(format!("Failed to upload image: {status}")),
        }
    }

    // ── Branch listing ────────────────────────────────────────────

    /// List all branches matching the `editor/` prefix.
    pub async fn list_editor_branches(&self) -> Result<Vec<GitRef>, String> {
        let url =
            format!("{API_BASE}/repos/{OWNER}/{REPO}/git/matching-refs/heads/editor/");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized \u{2014} check your token".into()),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    // ── CI status ───────────────────────────────────────────────

    /// Get check runs for a given git ref (branch name or SHA).
    pub async fn get_check_runs(&self, git_ref: &str) -> Result<CheckRunsResponse, String> {
        let url =
            format!("{API_BASE}/repos/{OWNER}/{REPO}/commits/{git_ref}/check-runs");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err(format!("Ref not found: {git_ref}")),
            status => Err(format!("GitHub API error: {status}")),
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
