use serde::{Deserialize, Serialize};

/// A single entry returned by the GitHub Contents API when listing a directory.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ContentEntry {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub sha: String,
    #[serde(default)]
    pub size: u64,
    pub download_url: Option<String>,
}

/// Full file content returned by the GitHub Contents API when reading a single file.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct FileContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    #[serde(default)]
    pub size: u64,
    pub content: Option<String>,
    pub encoding: Option<String>,
}

/// A git reference (branch pointer) from the Git Refs API.
#[derive(Clone, Debug, Deserialize)]
pub struct GitRef {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub object: GitObject,
}

/// The object a git ref points to.
#[derive(Clone, Debug, Deserialize)]
pub struct GitObject {
    pub sha: String,
}

/// Response from the Git Trees API (`GET /repos/{owner}/{repo}/git/trees/{sha}`).
#[derive(Clone, Debug, Deserialize)]
pub struct TreeResponse {
    pub tree: Vec<TreeEntry>,
}

/// A single entry in a git tree.
#[derive(Clone, Debug, Deserialize)]
pub struct TreeEntry {
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub sha: String,
    #[serde(default)]
    pub size: Option<u64>,
}

/// Response from the GitHub Compare API (`GET /repos/{owner}/{repo}/compare/{base}...{head}`).
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CompareResponse {
    pub status: String,
    pub ahead_by: u32,
    pub total_commits: u32,
    #[serde(default)]
    pub files: Vec<DiffFile>,
}

/// A single file entry in a compare response.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DiffFile {
    pub filename: String,
    pub status: String,
    pub additions: u32,
    pub deletions: u32,
    pub changes: u32,
    pub patch: Option<String>,
}

/// A commit from the GitHub Commits API (minimal fields for last-modified sorting).
#[derive(Clone, Debug, Deserialize)]
pub struct CommitInfo {
    pub commit: CommitDetail,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommitDetail {
    pub committer: CommitAuthor,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommitAuthor {
    pub date: String,
}

/// Response from the GitHub Check Runs API.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckRunsResponse {
    pub check_runs: Vec<CheckRun>,
}

/// A single check run from the Check Runs API.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckRun {
    pub status: String,
    pub conclusion: Option<String>,
    pub html_url: String,
}
