use serde::Deserialize;

/// A single entry returned by the GitHub Contents API when listing a directory.
#[derive(Clone, Debug, PartialEq, Deserialize)]
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
