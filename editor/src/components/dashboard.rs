use std::cmp::Ordering;

use gloo_storage::{SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::models::github::{ContentEntry, GitRef};
use crate::routes::Route;
use crate::services::github::GitHubClient;

const BRANCH_KEY: &str = "editor_branch";
const CACHE_TTL_MS: f64 = 5.0 * 60.0 * 1000.0; // 5 minutes

// ── Directory listing cache ─────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct CachedListing {
    entries: Vec<ContentEntry>,
    timestamp: f64,
}

fn cache_key(path: &str) -> String {
    format!("dir_cache_{path}")
}

fn get_cached_listing(path: &str) -> Option<Vec<ContentEntry>> {
    let cached: CachedListing = SessionStorage::get(&cache_key(path)).ok()?;
    let now = js_sys::Date::now();
    if now - cached.timestamp < CACHE_TTL_MS {
        Some(cached.entries)
    } else {
        SessionStorage::delete(&cache_key(path));
        None
    }
}

fn set_cached_listing(path: &str, entries: &[ContentEntry]) {
    let cached = CachedListing {
        entries: entries.to_vec(),
        timestamp: js_sys::Date::now(),
    };
    let _ = SessionStorage::set(&cache_key(path), &cached);
}

/// Invalidate a single directory cache entry.
pub fn invalidate_cache(path: &str) {
    SessionStorage::delete(&cache_key(path));
}

/// Invalidate all directory cache entries.
pub fn invalidate_all_caches() {
    let storage = gloo_utils::window()
        .session_storage()
        .ok()
        .flatten();
    let Some(storage) = storage else { return };
    let len = storage.length().unwrap_or(0);
    let mut keys_to_delete = Vec::new();
    for i in 0..len {
        if let Ok(Some(key)) = storage.key(i) {
            if key.starts_with("dir_cache_") {
                keys_to_delete.push(key);
            }
        }
    }
    for key in keys_to_delete {
        let _ = storage.delete(&key);
    }
}

// ── Dashboard component ─────────────────────────────────────────

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");

    let current_path = use_state(|| "content".to_string());
    let entries = use_state(|| Vec::<ContentEntry>::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let show_new_post = use_state(|| false);
    let new_section = use_state(|| "blog".to_string());
    let new_slug = use_state(String::new);
    let force_refresh = use_state(|| 0u32);

    // Branch selector state
    let branches = use_state(|| Vec::<GitRef>::new());
    let show_branches = use_state(|| false);
    let active_branch = use_state(|| -> Option<String> {
        SessionStorage::get(BRANCH_KEY).ok()
    });

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

    // Fetch editor branches on mount
    {
        let branches = branches.clone();
        let token = auth.token.clone();

        use_effect_with(token.clone(), move |_| {
            if let Some(token) = token {
                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    if let Ok(refs) = client.list_editor_branches().await {
                        branches.set(refs);
                    }
                });
            }
            || ()
        });
    }

    // Fetch contents when path, token, branch, or force_refresh changes
    {
        let entries = entries.clone();
        let loading = loading.clone();
        let error = error.clone();
        let path = (*current_path).clone();
        let token = auth.token.clone();
        let set_token = auth.set_token.clone();
        let refresh = *force_refresh;
        let branch = (*active_branch).clone();

        use_effect_with(
            (path.clone(), token.clone(), refresh, branch.clone()),
            move |_| {
                if let Some(token) = token {
                    // Check cache (skip on force refresh)
                    let use_cache = refresh == 0;
                    let cached = if use_cache {
                        get_cached_listing(&path)
                    } else {
                        None
                    };

                    if let Some(cached) = cached {
                        entries.set(cached);
                        loading.set(false);
                    } else {
                        loading.set(true);
                        error.set(None);

                        wasm_bindgen_futures::spawn_local(async move {
                            let client = GitHubClient::new(token);
                            match client.list_contents(&path, branch.as_deref()).await {
                                Ok(mut items) => {
                                    items.sort_by(|a, b| {
                                        let type_ord = match (
                                            a.entry_type.as_str(),
                                            b.entry_type.as_str(),
                                        ) {
                                            ("dir", "file") => Ordering::Less,
                                            ("file", "dir") => Ordering::Greater,
                                            _ => Ordering::Equal,
                                        };
                                        type_ord.then_with(|| a.name.cmp(&b.name))
                                    });
                                    set_cached_listing(&path, &items);
                                    entries.set(items);
                                    loading.set(false);
                                }
                                Err(e) => {
                                    if e.contains("Unauthorized") {
                                        set_token.emit(None);
                                    }
                                    error.set(Some(e));
                                    entries.set(vec![]);
                                    loading.set(false);
                                }
                            }
                        });
                    }
                }

                || ()
            },
        );
    }

    let on_refresh = {
        let force_refresh = force_refresh.clone();
        Callback::from(move |_: MouseEvent| {
            invalidate_all_caches();
            force_refresh.set(*force_refresh + 1);
        })
    };

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

    let toggle_new_post = {
        let show_new_post = show_new_post.clone();
        Callback::from(move |_: MouseEvent| {
            show_new_post.set(!*show_new_post);
        })
    };

    let on_section_input = {
        let new_section = new_section.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            new_section.set(target.value());
        })
    };

    let on_slug_input = {
        let new_slug = new_slug.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            new_slug.set(target.value());
        })
    };

    let on_create_post = {
        let navigator = navigator.clone();
        let new_section = new_section.clone();
        let new_slug = new_slug.clone();
        let show_new_post = show_new_post.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let section = (*new_section).clone();
            let slug = (*new_slug).clone();
            if !slug.is_empty() {
                let path = format!("content/{section}/{slug}/index.md");
                show_new_post.set(false);
                navigator.push(&Route::Editor { path });
            }
        })
    };

    let toggle_branches = {
        let show_branches = show_branches.clone();
        Callback::from(move |_: MouseEvent| {
            show_branches.set(!*show_branches);
        })
    };

    let on_select_branch = {
        let active_branch = active_branch.clone();
        let force_refresh = force_refresh.clone();
        Callback::from(move |name: String| {
            let _ = SessionStorage::set(BRANCH_KEY, &name);
            active_branch.set(Some(name));
            invalidate_all_caches();
            force_refresh.set(*force_refresh + 1);
        })
    };

    let on_clear_branch = {
        let active_branch = active_branch.clone();
        let force_refresh = force_refresh.clone();
        Callback::from(move |_: MouseEvent| {
            SessionStorage::delete(BRANCH_KEY);
            active_branch.set(None);
            invalidate_all_caches();
            force_refresh.set(*force_refresh + 1);
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
                <div class="dashboard-title-row">
                    <h2>{"Content"}</h2>
                    <div class="dashboard-actions">
                        <button class="refresh-btn" onclick={on_refresh} title="Refresh">
                            {"\u{21BB}"}
                        </button>
                        <button class="branch-toggle-btn" onclick={toggle_branches}>
                            { if *show_branches { "Hide branches" } else { "Branches" } }
                        </button>
                        <button class="new-post-btn" onclick={toggle_new_post}>
                            { if *show_new_post { "Cancel" } else { "+ New Post" } }
                        </button>
                    </div>
                </div>
                if let Some(ref name) = *active_branch {
                    <div class="active-branch-badge">
                        <span class="branch-label">{format!("Branch: {name}")}</span>
                        <button class="clear-branch-btn" onclick={on_clear_branch.clone()}>
                            {"\u{00D7}"}
                        </button>
                    </div>
                }
                if *show_branches {
                    {render_branch_list(&branches, &active_branch, on_select_branch.clone(), on_clear_branch.clone())}
                }
                <div class="breadcrumbs">{breadcrumbs}</div>
            </div>

            if *show_new_post {
                <form class="new-post-form" onsubmit={on_create_post}>
                    <div class="form-row">
                        <label for="section">{"Section"}</label>
                        <input
                            id="section"
                            type="text"
                            value={(*new_section).clone()}
                            oninput={on_section_input}
                            placeholder="blog"
                        />
                    </div>
                    <div class="form-row">
                        <label for="slug">{"Slug"}</label>
                        <input
                            id="slug"
                            type="text"
                            value={(*new_slug).clone()}
                            oninput={on_slug_input}
                            placeholder="my-new-post"
                        />
                    </div>
                    <button type="submit" class="create-btn" disabled={new_slug.is_empty()}>
                        {"Create"}
                    </button>
                    <span class="new-post-preview">
                        {format!("content/{}/{}/index.md", *new_section, *new_slug)}
                    </span>
                </form>
            }

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

// ── Branch list rendering ───────────────────────────────────────

fn render_branch_list(
    branches: &UseStateHandle<Vec<GitRef>>,
    active_branch: &UseStateHandle<Option<String>>,
    on_select: Callback<String>,
    on_clear: Callback<MouseEvent>,
) -> Html {
    if branches.is_empty() {
        return html! {
            <div class="branch-list">
                <p class="branch-empty">{"No editor branches found."}</p>
            </div>
        };
    }

    html! {
        <div class="branch-list">
            <div class="branch-list-header">
                <span class="branch-list-title">{"Editor branches"}</span>
                if active_branch.is_some() {
                    <button class="clear-branch-btn" onclick={on_clear}>
                        {"View source"}
                    </button>
                }
            </div>
            { for (**branches).iter().map(|git_ref| {
                let name = git_ref.ref_name.strip_prefix("refs/heads/").unwrap_or(&git_ref.ref_name).to_string();
                let display = name.strip_prefix("editor/").unwrap_or(&name).to_string();
                let is_active = active_branch.as_ref().map_or(false, |b| *b == name);
                let on_select = on_select.clone();
                let name_clone = name.clone();
                let onclick = Callback::from(move |_: MouseEvent| {
                    on_select.emit(name_clone.clone());
                });
                html! {
                    <div class={classes!("branch-item", is_active.then_some("active"))} onclick={onclick}>
                        <span class="branch-name">{&display}</span>
                        if is_active {
                            <span class="branch-active-indicator">{"current"}</span>
                        }
                    </div>
                }
            }) }
        </div>
    }
}

// ── Entry rendering ─────────────────────────────────────────────

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
