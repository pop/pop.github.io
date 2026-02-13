use gloo_net::http::Request;
use gloo_storage::{SessionStorage, Storage};
use serde::Deserialize;

const TOKEN_KEY: &str = "github_token";

// TODO: Set this to your deployed Cloudflare Worker URL
const WORKER_URL: &str = "";

pub fn get_token() -> Option<String> {
    SessionStorage::get(TOKEN_KEY).ok()
}

pub fn store_token(token: &str) {
    let _ = SessionStorage::set(TOKEN_KEY, token);
}

pub fn clear_token() {
    SessionStorage::delete(TOKEN_KEY);
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

pub async fn exchange_code(code: &str) -> Result<String, String> {
    if WORKER_URL.is_empty() {
        return Err("OAuth worker URL not configured".into());
    }

    let resp = Request::post(&format!("{WORKER_URL}/exchange"))
        .json(&serde_json::json!({ "code": code }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err(format!("Worker returned status {}", resp.status()));
    }

    let body: TokenResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.access_token)
}
