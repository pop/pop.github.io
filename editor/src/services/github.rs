/// GitHub API client — implemented in Phase 2.
pub struct GitHubClient {
    pub token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
