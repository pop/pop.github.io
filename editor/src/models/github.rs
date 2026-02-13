/// GitHub API response types — implemented in Phase 2.
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ContentEntry {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub sha: String,
}
