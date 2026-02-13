use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Deserialize)]
struct CodeRequest {
    code: String,
}

#[derive(Deserialize)]
struct GitHubTokenResponse {
    access_token: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn cors_headers() -> Headers {
    let mut headers = Headers::new();
    let _ = headers.set("Access-Control-Allow-Origin", "*");
    let _ = headers.set("Access-Control-Allow-Methods", "POST, OPTIONS");
    let _ = headers.set("Access-Control-Allow-Headers", "Content-Type");
    headers
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // CORS preflight
    if req.method() == Method::Options {
        return Ok(Response::empty()?.with_headers(cors_headers()));
    }

    if req.method() != Method::Post || req.path() != "/exchange" {
        return Response::error("Not Found", 404);
    }

    let result = handle_exchange(req, &env).await;
    let mut resp = match result {
        Ok(r) => r,
        Err(e) => Response::from_json(&ErrorResponse {
            error: e.to_string(),
        })
        .unwrap()
        .with_status(500),
    };

    let headers = cors_headers();
    for (key, value) in headers.entries() {
        resp.headers_mut().set(&key, &value)?;
    }
    Ok(resp)
}

async fn handle_exchange(mut req: Request, env: &Env) -> Result<Response> {
    let body: CodeRequest = req.json().await?;
    let client_id = env.secret("GITHUB_CLIENT_ID")?.to_string();
    let client_secret = env.secret("GITHUB_CLIENT_SECRET")?.to_string();

    let github_body = serde_json::json!({
        "client_id": client_id,
        "client_secret": client_secret,
        "code": body.code,
    });

    let mut headers = Headers::new();
    headers.set("Accept", "application/json")?;
    headers.set("Content-Type", "application/json")?;
    headers.set("User-Agent", "elijah-run-editor")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);
    init.with_body(Some(wasm_bindgen::JsValue::from_str(
        &github_body.to_string(),
    )));

    let github_req = Request::new_with_init(
        "https://github.com/login/oauth/access_token",
        &init,
    )?;

    let mut github_resp = Fetch::Request(github_req).send().await?;
    let token_resp: GitHubTokenResponse = github_resp.json().await?;

    Response::from_json(&TokenResponse {
        access_token: token_resp.access_token,
    })
}
