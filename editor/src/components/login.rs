use wasm_bindgen::JsCast;
use web_sys::UrlSearchParams;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::routes::Route;
use crate::services::auth;

// TODO: Set this to your GitHub OAuth App's client ID
const GITHUB_CLIENT_ID: &str = "";

#[function_component(Login)]
pub fn login() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");
    let dev_token = use_state(String::new);
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);

    // Redirect to dashboard if already authenticated
    {
        let navigator = navigator.clone();
        let token = auth.token.clone();
        use_effect_with(token, move |token| {
            if token.is_some() {
                navigator.push(&Route::Dashboard);
            }
            || ()
        });
    }

    // Handle OAuth callback: detect ?code= and exchange for token
    {
        let set_token = auth.set_token.clone();
        let error = error.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            let window = gloo_utils::window();
            let search = window.location().search().unwrap_or_default();
            if let Ok(params) = UrlSearchParams::new_with_str(&search) {
                if let Some(code) = params.get("code") {
                    loading.set(true);
                    wasm_bindgen_futures::spawn_local(async move {
                        match auth::exchange_code(&code).await {
                            Ok(token) => {
                                set_token.emit(Some(token));
                                // Clear ?code= from the URL
                                if let Ok(history) = window.history() {
                                    let _ = history.replace_state_with_url(
                                        &wasm_bindgen::JsValue::NULL,
                                        "",
                                        Some("/"),
                                    );
                                }
                            }
                            Err(e) => {
                                error.set(Some(format!("OAuth error: {e}")));
                                loading.set(false);
                            }
                        }
                    });
                }
            }
            || ()
        });
    }

    let on_github_login = Callback::from(|_: MouseEvent| {
        let origin = gloo_utils::window()
            .location()
            .origin()
            .unwrap_or_default();
        let url = format!(
            "https://github.com/login/oauth/authorize?client_id={GITHUB_CLIENT_ID}&redirect_uri={origin}&scope=repo"
        );
        let _ = gloo_utils::window().location().set_href(&url);
    });

    let on_dev_token_input = {
        let dev_token = dev_token.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target().unwrap().unchecked_into();
            dev_token.set(input.value());
        })
    };

    let on_dev_login = {
        let dev_token = dev_token.clone();
        let set_token = auth.set_token.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let token = (*dev_token).clone();
            if !token.is_empty() {
                set_token.emit(Some(token));
            }
        })
    };

    html! {
        <div class="login-page">
            <h1>{"Blog Editor"}</h1>
            <p>{"Sign in to edit elijah.run"}</p>

            if *loading {
                <p class="loading">{"Exchanging OAuth code…"}</p>
            } else {
                if let Some(err) = &*error {
                    <p class="error">{err}</p>
                }

                <div class="login-options">
                    <button onclick={on_github_login} class="github-btn">
                        {"Login with GitHub"}
                    </button>

                    <div class="dev-login">
                        <details>
                            <summary>{"Dev mode: enter token manually"}</summary>
                            <form onsubmit={on_dev_login}>
                                <input
                                    type="password"
                                    placeholder="GitHub personal access token"
                                    value={(*dev_token).clone()}
                                    oninput={on_dev_token_input}
                                />
                                <button type="submit">{"Use Token"}</button>
                            </form>
                        </details>
                    </div>
                </div>
            }
        </div>
    }
}
