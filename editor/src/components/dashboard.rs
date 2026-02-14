use std::cmp::Ordering;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::models::github::ContentEntry;
use crate::routes::Route;
use crate::services::github::GitHubClient;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");

    let current_path = use_state(|| "content".to_string());
    let entries = use_state(|| Vec::<ContentEntry>::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    // Redirect to login if not authenticated
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

    // Fetch contents when path or token changes
    {
        let entries = entries.clone();
        let loading = loading.clone();
        let error = error.clone();
        let path = (*current_path).clone();
        let token = auth.token.clone();

        use_effect_with((path.clone(), token.clone()), move |_| {
            if let Some(token) = token {
                loading.set(true);
                error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    match client.list_contents(&path).await {
                        Ok(mut items) => {
                            items.sort_by(|a, b| {
                                let type_ord =
                                    match (a.entry_type.as_str(), b.entry_type.as_str()) {
                                        ("dir", "file") => Ordering::Less,
                                        ("file", "dir") => Ordering::Greater,
                                        _ => Ordering::Equal,
                                    };
                                type_ord.then_with(|| a.name.cmp(&b.name))
                            });
                            entries.set(items);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e));
                            entries.set(vec![]);
                            loading.set(false);
                        }
                    }
                });
            }

            || ()
        });
    }

    let on_navigate = {
        let current_path = current_path.clone();
        let navigator = navigator.clone();
        Callback::from(move |entry: ContentEntry| {
            if entry.entry_type == "dir" {
                current_path.set(entry.path);
            } else {
                navigator.push(&Route::Editor {
                    path: entry.path,
                });
            }
        })
    };

    let on_navigate_up = {
        let current_path = current_path.clone();
        Callback::from(move |_: MouseEvent| {
            let path = (*current_path).clone();
            if let Some(pos) = path.rfind('/') {
                current_path.set(path[..pos].to_string());
            }
        })
    };

    let breadcrumbs = render_breadcrumbs(&current_path, {
        let current_path = current_path.clone();
        Callback::from(move |path: String| {
            current_path.set(path);
        })
    });

    html! {
        <div class="dashboard">
            <div class="dashboard-header">
                <h2>{"Content"}</h2>
                <div class="breadcrumbs">{breadcrumbs}</div>
            </div>

            if *current_path != "content" {
                <div class="navigate-up">
                    <a onclick={on_navigate_up} class="up-link">{"\u{2190} Back"}</a>
                </div>
            }

            if *loading {
                <p class="loading">{"Loading\u{2026}"}</p>
            } else if let Some(err) = &*error {
                <p class="error">{err}</p>
            } else if entries.is_empty() {
                <p class="empty">{"This directory is empty."}</p>
            } else {
                <div class="content-list">
                    { for (*entries).iter().map(|entry| {
                        render_entry(entry, on_navigate.clone())
                    }) }
                </div>
            }
        </div>
    }
}

fn render_entry(entry: &ContentEntry, on_click: Callback<ContentEntry>) -> Html {
    let is_dir = entry.entry_type == "dir";
    let entry_clone = entry.clone();
    let onclick = Callback::from(move |_: MouseEvent| {
        on_click.emit(entry_clone.clone());
    });

    html! {
        <div class={classes!("content-entry", is_dir.then_some("is-dir"))} onclick={onclick}>
            <span class="entry-icon">
                { if is_dir { "\u{25B8}" } else { "\u{00B7}" } }
            </span>
            <span class="entry-name">{&entry.name}</span>
            if !is_dir {
                <span class="entry-size">{format_size(entry.size)}</span>
            }
        </div>
    }
}

fn render_breadcrumbs(
    current_path: &UseStateHandle<String>,
    on_navigate: Callback<String>,
) -> Html {
    let path = (**current_path).clone();
    let parts: Vec<&str> = path.split('/').collect();

    html! {
        { for parts.iter().enumerate().map(|(i, part)| {
            let is_last = i == parts.len() - 1;
            if is_last {
                html! { <span class="breadcrumb-current">{*part}</span> }
            } else {
                let target = parts[..=i].join("/");
                let on_navigate = on_navigate.clone();
                let onclick = Callback::from(move |_: MouseEvent| {
                    on_navigate.emit(target.clone());
                });
                html! {
                    <>
                        <a class="breadcrumb-link" onclick={onclick}>{*part}</a>
                        <span class="breadcrumb-sep">{"/"}</span>
                    </>
                }
            }
        }) }
    }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
