use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::models::post::render_markdown;
use crate::routes::Route;
use crate::services::github::{decode_github_content, GitHubClient};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: String,
}

#[function_component(Preview)]
pub fn preview(props: &Props) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");

    let content = use_state(String::new);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Redirect if not authenticated
    {
        let navigator = navigator.clone();
        let token = auth.token.clone();
        use_effect_with(token.clone(), move |token| {
            if token.is_none() {
                navigator.push(&Route::Login);
            }
            || ()
        });
    }

    // Load file content from source branch
    {
        let content = content.clone();
        let loading = loading.clone();
        let error = error.clone();
        let path = props.path.clone();
        let token = auth.token.clone();
        let set_token = auth.set_token.clone();

        use_effect_with((path.clone(), token.clone()), move |_| {
            if let Some(token) = token {
                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    match client.get_file(&path, "source").await {
                        Ok(file) => {
                            let text =
                                decode_github_content(&file.content.unwrap_or_default());
                            content.set(text);
                            loading.set(false);
                        }
                        Err(e) => {
                            if e.contains("Unauthorized") {
                                set_token.emit(None);
                            }
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                });
            }
            || ()
        });
    }

    html! {
        <div class="preview-page">
            <div class="preview-nav">
                <Link<Route> to={Route::Dashboard} classes="back-link">
                    {"\u{2190} Dashboard"}
                </Link<Route>>
            </div>
            <div class="preview-header">
                <h2>{&props.path}</h2>
                <Link<Route> to={Route::Editor { path: props.path.clone() }} classes="edit-link">
                    {"Edit"}
                </Link<Route>>
            </div>

            if *loading {
                <p class="loading">{"Loading\u{2026}"}</p>
            } else if let Some(ref err) = *error {
                <p class="error">{err}</p>
            } else {
                <div class="preview-pane markdown-body">
                    {Html::from_html_unchecked(AttrValue::from(render_markdown(&content)))}
                </div>
            }
        </div>
    }
}
