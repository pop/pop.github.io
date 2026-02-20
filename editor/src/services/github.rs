use std::collections::HashMap;

use base64::prelude::*;
use futures::future::join_all;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::models::github::{
    CheckRunsResponse, CommitInfo, CompareResponse, ContentEntry, FileContent,
    GqlBlobData, GqlRefData, GqlRefsData, GqlTreeData, GitRef, GraphQLResponse, TreeResponse,
};
use crate::models::post::{bytes_to_data_url, extract_relative_image_srcs, post_dir, replace_image_srcs};

const OWNER: &str = "pop";
const REPO: &str = "pop.github.io";
const API_BASE: &str = "https://api.github.com";
const GRAPHQL_URL: &str = "https://api.github.com/graphql";

pub struct GitHubClient {
    pub token: Option<String>,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self { token: Some(token) }
    }

    pub fn anonymous() -> Self {
        Self { token: None }
    }

    fn require_token(&self) -> Result<&str, String> {
        self.token
            .as_deref()
            .ok_or_else(|| "Authentication required for this operation".into())
    }

    // ── Directory & file reading ─────────────────────────────────

    /// List the contents of a directory in the repo.
    ///
    /// Uses GraphQL when authenticated (no entry count limit).
    /// Falls back to REST when anonymous, with a Git Trees API fallback
    /// if the Contents API returns 1000 entries (its hard cap).
    pub async fn list_contents(
        &self,
        path: &str,
        branch: Option<&str>,
    ) -> Result<Vec<ContentEntry>, String> {
        if self.token.is_some() {
            self.list_contents_graphql(path, branch).await
        } else {
            self.list_contents_rest(path, branch).await
        }
    }

    async fn list_contents_graphql(
        &self,
        path: &str,
        branch: Option<&str>,
    ) -> Result<Vec<ContentEntry>, String> {
        let git_ref = branch.unwrap_or("source");
        let expression = format!("{git_ref}:{path}");

        let query = r#"
            query($owner: String!, $name: String!, $expression: String!) {
                repository(owner: $owner, name: $name) {
                    object(expression: $expression) {
                        ... on Tree {
                            entries {
                                name
                                type
                                oid
                                object {
                                    ... on Blob { byteSize }
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let data: GqlTreeData = self
            .graphql(query, json!({ "owner": OWNER, "name": REPO, "expression": expression }))
            .await?;

        let tree = data
            .repository
            .object
            .ok_or_else(|| format!("Path not found: {path}"))?;

        let entries = tree.entries.unwrap_or_default();

        Ok(entries
            .into_iter()
            .map(|e| ContentEntry {
                path: if path.is_empty() {
                    e.name.clone()
                } else {
                    format!("{path}/{}", e.name)
                },
                name: e.name,
                entry_type: if e.entry_type == "tree" { "dir" } else { "file" }.to_string(),
                sha: e.oid,
                size: e.object.and_then(|o| o.byte_size).unwrap_or(0),
                download_url: None,
            })
            .collect())
    }

    async fn list_contents_rest(
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
    ///
    /// Returns decoded text content (not base64) regardless of API path.
    pub async fn get_file(&self, path: &str, branch: &str) -> Result<FileContent, String> {
        if self.token.is_some() {
            self.get_file_graphql(path, branch).await
        } else {
            self.get_file_rest(path, branch).await
        }
    }

    async fn get_file_graphql(
        &self,
        path: &str,
        branch: &str,
    ) -> Result<FileContent, String> {
        let expression = format!("{branch}:{path}");

        let query = r#"
            query($owner: String!, $name: String!, $expression: String!) {
                repository(owner: $owner, name: $name) {
                    object(expression: $expression) {
                        ... on Blob {
                            text
                            oid
                            byteSize
                        }
                    }
                }
            }
        "#;

        let data: GqlBlobData = self
            .graphql(query, json!({ "owner": OWNER, "name": REPO, "expression": expression }))
            .await?;

        let blob = data
            .repository
            .object
            .ok_or_else(|| format!("File not found: {path}"))?;

        let name = path.rsplit('/').next().unwrap_or(path).to_string();

        Ok(FileContent {
            name,
            path: path.to_string(),
            sha: blob.oid,
            size: blob.byte_size,
            content: blob.text,
            encoding: None,
        })
    }

    async fn get_file_rest(
        &self,
        path: &str,
        branch: &str,
    ) -> Result<FileContent, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref={branch}");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => {
                let mut fc: FileContent = resp.json().await.map_err(|e| e.to_string())?;
                // Decode base64 content so callers always get plaintext
                fc.content = fc.content.map(|c| decode_github_content(&c));
                Ok(fc)
            }
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err(format!("File not found: {path}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    /// Read a single file from the repo and return its raw bytes.
    ///
    /// Uses the REST Contents API. The base64 content from the response is
    /// stripped of whitespace before decoding, matching GitHub's encoding.
    pub async fn get_file_bytes(&self, path: &str, branch: &str) -> Result<Vec<u8>, String> {
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}?ref={branch}");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => {
                let fc: FileContent = resp.json().await.map_err(|e| e.to_string())?;
                let encoded = fc.content.unwrap_or_default();
                let cleaned: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
                BASE64_STANDARD.decode(cleaned.as_bytes()).map_err(|e| e.to_string())
            }
            401 => Err("Unauthorized \u{2014} check your token".into()),
            404 => Err(format!("File not found: {path}")),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    /// Resolve relative image `src` attributes in rendered HTML to data URLs.
    ///
    /// Fetches each relative image from the repo using `get_file_bytes` (in parallel),
    /// converts successful results to `data:` URLs, and returns the HTML with
    /// all resolved `src` values substituted. Images that fail to fetch (404, error)
    /// retain their original relative `src`.
    pub async fn resolve_images_in_html(
        &self,
        html: &str,
        post_path: &str,
        branch: &str,
    ) -> String {
        let srcs = extract_relative_image_srcs(html);
        if srcs.is_empty() {
            return html.to_string();
        }

        let dir = post_dir(post_path);
        let branch = branch.to_string();

        let futures: Vec<_> = srcs
            .iter()
            .map(|src| {
                let repo_path = if dir.is_empty() {
                    src.clone()
                } else {
                    format!("{dir}/{src}")
                };
                let src = src.clone();
                let branch = branch.clone();
                async move {
                    match self.get_file_bytes(&repo_path, &branch).await {
                        Ok(bytes) => (src.clone(), bytes_to_data_url(&bytes, &src)),
                        Err(_) => (src.clone(), src),
                    }
                }
            })
            .collect();

        let replacements: HashMap<String, String> = join_all(futures).await.into_iter().collect();
        replace_image_srcs(html, &replacements)
    }

    // ── Branch operations ────────────────────────────────────────

    /// Get the HEAD SHA of a branch.
    pub async fn get_branch_sha(&self, branch: &str) -> Result<String, String> {
        if self.token.is_some() {
            self.get_branch_sha_graphql(branch).await
        } else {
            self.get_branch_sha_rest(branch).await
        }
    }

    async fn get_branch_sha_graphql(&self, branch: &str) -> Result<String, String> {
        let qualified = format!("refs/heads/{branch}");

        let query = r#"
            query($owner: String!, $name: String!, $ref: String!) {
                repository(owner: $owner, name: $name) {
                    ref(qualifiedName: $ref) {
                        target { oid }
                    }
                }
            }
        "#;

        let data: GqlRefData = self
            .graphql(query, json!({ "owner": OWNER, "name": REPO, "ref": qualified }))
            .await?;

        data.repository
            .git_ref
            .map(|r| r.target.oid)
            .ok_or_else(|| format!("Branch not found: {branch}"))
    }

    async fn get_branch_sha_rest(&self, branch: &str) -> Result<String, String> {
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
        let token = self.require_token()?;
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/refs");
        let body = json!({
            "ref": format!("refs/heads/{branch_name}"),
            "sha": from_sha,
        });

        let resp = Request::post(&url)
            .header("Authorization", &format!("Bearer {token}"))
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
        let token = self.require_token()?;
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/refs/heads/{branch_name}");
        let resp = Request::delete(&url)
            .header("Authorization", &format!("Bearer {token}"))
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

        let token = self.require_token()?;
        let resp = Request::post(&url)
            .header("Authorization", &format!("Bearer {token}"))
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
        let token = self.require_token()?;
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
            .header("Authorization", &format!("Bearer {token}"))
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
        let token = self.require_token()?;
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/contents/{path}");
        let body = json!({
            "message": message,
            "sha": sha,
            "branch": branch,
        });

        let resp = Request::delete(&url)
            .header("Authorization", &format!("Bearer {token}"))
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
        let token = self.require_token()?;
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
            .header("Authorization", &format!("Bearer {token}"))
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
        if self.token.is_some() {
            self.list_editor_branches_graphql().await
        } else {
            self.list_editor_branches_rest().await
        }
    }

    async fn list_editor_branches_graphql(&self) -> Result<Vec<GitRef>, String> {
        let query = r#"
            query($owner: String!, $name: String!) {
                repository(owner: $owner, name: $name) {
                    refs(refPrefix: "refs/heads/editor/", first: 100) {
                        nodes {
                            name
                            prefix
                            target { oid }
                        }
                    }
                }
            }
        "#;

        let data: GqlRefsData = self
            .graphql(query, json!({ "owner": OWNER, "name": REPO }))
            .await?;

        let nodes = data
            .repository
            .refs
            .map(|r| r.nodes)
            .unwrap_or_default();

        Ok(nodes
            .into_iter()
            .map(|n| GitRef {
                ref_name: format!("{}{}", n.prefix, n.name),
                object: crate::models::github::GitObject { sha: n.target.oid },
            })
            .collect())
    }

    async fn list_editor_branches_rest(&self) -> Result<Vec<GitRef>, String> {
        let url =
            format!("{API_BASE}/repos/{OWNER}/{REPO}/git/matching-refs/heads/editor/");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => resp.json().await.map_err(|e| e.to_string()),
            401 => Err("Unauthorized \u{2014} check your token".into()),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    // ── Commit dates ─────────────────────────────────────────────

    /// Get the last commit date for a file path on a branch.
    pub async fn get_last_commit_date(
        &self,
        file_path: &str,
        branch: Option<&str>,
    ) -> Result<String, String> {
        let git_ref = branch.unwrap_or("source");
        let url = format!(
            "{API_BASE}/repos/{OWNER}/{REPO}/commits?path={file_path}&sha={git_ref}&per_page=1"
        );
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => {
                let commits: Vec<CommitInfo> =
                    resp.json().await.map_err(|e| e.to_string())?;
                commits
                    .first()
                    .map(|c| c.commit.committer.date.clone())
                    .ok_or_else(|| "No commits found".to_string())
            }
            401 => Err("Unauthorized \u{2014} check your token".into()),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    /// Fetch last commit dates for multiple file paths in parallel.
    /// Returns a map of path -> epoch milliseconds.
    /// Entries that fail to fetch are silently omitted.
    pub async fn get_commit_dates_bulk(
        &self,
        paths: &[String],
        branch: Option<&str>,
    ) -> HashMap<String, f64> {
        let futures: Vec<_> = paths
            .iter()
            .map(|path| {
                let path = path.clone();
                let token = self.token.clone();
                let branch = branch.map(|b| b.to_string());
                async move {
                    let client = match token {
                        Some(t) => GitHubClient::new(t),
                        None => GitHubClient::anonymous(),
                    };
                    match client
                        .get_last_commit_date(&path, branch.as_deref())
                        .await
                    {
                        Ok(date_str) => {
                            let ms = js_sys::Date::parse(&date_str);
                            if ms.is_nan() {
                                None
                            } else {
                                Some((path, ms))
                            }
                        }
                        Err(_) => None,
                    }
                }
            })
            .collect();

        let results = join_all(futures).await;
        results.into_iter().flatten().collect()
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

    // ── Global file index ────────────────────────────────────────

    /// List every file (blob) in the repo using the recursive Git Trees API.
    pub async fn list_all_files(&self, branch: Option<&str>) -> Result<Vec<ContentEntry>, String> {
        let sha = self.get_branch_sha(branch.unwrap_or("source")).await?;
        let url = format!("{API_BASE}/repos/{OWNER}/{REPO}/git/trees/{sha}?recursive=1");
        let resp = self.get(&url).await?;

        match resp.status() {
            200 => {
                let tree: TreeResponse = resp.json().await.map_err(|e| e.to_string())?;
                let entries = tree
                    .tree
                    .into_iter()
                    .filter(|te| te.entry_type == "blob" && te.path.starts_with("content/"))
                    .map(|te| {
                        let name = te
                            .path
                            .rsplit('/')
                            .next()
                            .unwrap_or(&te.path)
                            .to_string();
                        ContentEntry {
                            name,
                            path: te.path,
                            entry_type: "file".to_string(),
                            sha: te.sha,
                            size: te.size.unwrap_or(0),
                            download_url: None,
                        }
                    })
                    .collect();
                Ok(entries)
            }
            401 => Err("Unauthorized \u{2014} check your token".into()),
            status => Err(format!("GitHub API error: {status}")),
        }
    }

    // ── Helpers ──────────────────────────────────────────────────

    async fn get(&self, url: &str) -> Result<gloo_net::http::Response, String> {
        let mut req = Request::get(url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "elijah-run-editor");
        if let Some(ref token) = self.token {
            req = req.header("Authorization", &format!("Bearer {token}"));
        }
        req.send().await.map_err(|e| e.to_string())
    }

    async fn graphql<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<T, String> {
        let token = self.require_token()?;
        let body = json!({ "query": query, "variables": variables });

        let resp = Request::post(GRAPHQL_URL)
            .header("Authorization", &format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .header("User-Agent", "elijah-run-editor")
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status() == 401 {
            return Err("Unauthorized \u{2014} check your token".into());
        }

        let gql: GraphQLResponse<T> = resp.json().await.map_err(|e| e.to_string())?;

        if let Some(errors) = gql.errors {
            let msg = errors
                .iter()
                .map(|e| e.message.as_str())
                .collect::<Vec<_>>()
                .join("; ");
            return Err(msg);
        }

        gql.data.ok_or_else(|| "No data in GraphQL response".into())
    }
}

/// Decode base64 file content from the GitHub REST API (which includes newlines).
fn decode_github_content(encoded: &str) -> String {
    let cleaned: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
    match BASE64_STANDARD.decode(cleaned.as_bytes()) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}
