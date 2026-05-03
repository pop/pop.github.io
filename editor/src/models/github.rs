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

// ── GraphQL types ────────────────────────────────────────────────

/// Generic GraphQL response envelope.
#[derive(Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

/// Response for a Tree query (list_contents).
#[derive(Deserialize)]
pub struct GqlTreeData {
    pub repository: GqlRepoTree,
}

#[derive(Deserialize)]
pub struct GqlRepoTree {
    pub object: Option<GqlTree>,
}

#[derive(Deserialize)]
pub struct GqlTree {
    pub entries: Option<Vec<GqlTreeEntry>>,
}

#[derive(Deserialize)]
pub struct GqlTreeEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub oid: String,
    pub object: Option<GqlEntryObject>,
}

#[derive(Deserialize)]
pub struct GqlEntryObject {
    #[serde(rename = "byteSize")]
    pub byte_size: Option<u64>,
}

/// Response for a Blob query (get_file).
#[derive(Deserialize)]
pub struct GqlBlobData {
    pub repository: GqlRepoBlob,
}

#[derive(Deserialize)]
pub struct GqlRepoBlob {
    pub object: Option<GqlBlob>,
}

#[derive(Deserialize)]
pub struct GqlBlob {
    pub text: Option<String>,
    pub oid: String,
    #[serde(rename = "byteSize")]
    pub byte_size: u64,
}

/// Response for a ref query (get_branch_sha).
#[derive(Deserialize)]
pub struct GqlRefData {
    pub repository: GqlRepoRef,
}

#[derive(Deserialize)]
pub struct GqlRepoRef {
    #[serde(rename = "ref")]
    pub git_ref: Option<GqlRef>,
}

#[derive(Deserialize)]
pub struct GqlRef {
    pub target: GqlRefTarget,
}

#[derive(Deserialize)]
pub struct GqlRefTarget {
    pub oid: String,
}

/// Response for a batched ref + blob query (get_branch_sha_and_file).
#[derive(Deserialize)]
pub struct GqlRefAndBlobData {
    pub repository: GqlRepoRefAndBlob,
}

#[derive(Deserialize)]
pub struct GqlRepoRefAndBlob {
    #[serde(rename = "ref")]
    pub git_ref: Option<GqlRef>,
    pub object: Option<GqlBlob>,
}

/// Response for a refs query (list_editor_branches).
#[derive(Deserialize)]
pub struct GqlRefsData {
    pub repository: GqlRepoRefs,
}

#[derive(Deserialize)]
pub struct GqlRepoRefs {
    pub refs: Option<GqlRefConnection>,
}

#[derive(Deserialize)]
pub struct GqlRefConnection {
    pub nodes: Vec<GqlRefNode>,
}

#[derive(Deserialize)]
pub struct GqlRefNode {
    pub name: String,
    pub prefix: String,
    pub target: GqlRefTarget,
}

// ── History query types ──────────────────────────────────────────

/// A commit entry returned by the history query for a specific file path.
#[derive(Clone, Debug, PartialEq)]
pub struct CommitSummary {
    pub sha: String,
    pub message: String,
    pub date: String,
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Deserialize)]
pub struct GqlHistoryData {
    pub repository: GqlRepoHistory,
}

#[derive(Deserialize)]
pub struct GqlRepoHistory {
    #[serde(rename = "ref")]
    pub git_ref: Option<GqlHistoryRef>,
}

#[derive(Deserialize)]
pub struct GqlHistoryRef {
    pub target: GqlHistoryTarget,
}

#[derive(Deserialize)]
pub struct GqlHistoryTarget {
    pub history: Option<GqlHistoryConnection>,
}

#[derive(Deserialize)]
pub struct GqlHistoryConnection {
    pub nodes: Vec<GqlHistoryNode>,
}

#[derive(Deserialize)]
pub struct GqlHistoryNode {
    pub oid: String,
    pub message: String,
    #[serde(rename = "committedDate")]
    pub committed_date: String,
    pub additions: u32,
    pub deletions: u32,
}
