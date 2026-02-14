use gloo_storage::{SessionStorage, Storage};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::routes::Route;
use crate::services::github::{decode_github_content, GitHubClient};

const DEFAULT_BRANCH: &str = "source";
const BRANCH_KEY: &str = "editor_branch";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: String,
}

#[function_component(EditorPage)]
pub fn editor_page(props: &Props) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");

    let content = use_state(String::new);
    let original_content = use_state(String::new);
    let file_sha = use_state(|| Option::<String>::None);
    let branch = use_state(get_active_branch);
    let loading = use_state(|| true);
    let saving = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let save_msg = use_state(|| Option::<String>::None);
    let is_new = use_state(|| false);

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

    // Load file content on mount
    {
        let content = content.clone();
        let original_content = original_content.clone();
        let file_sha = file_sha.clone();
        let loading = loading.clone();
        let error = error.clone();
        let is_new = is_new.clone();
        let branch = branch.clone();
        let path = props.path.clone();
        let token = auth.token.clone();

        use_effect_with((path.clone(), token.clone()), move |_| {
            if let Some(token) = token {
                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    let active_branch = (*branch).clone();

                    let result = load_file(&client, &path, active_branch.as_deref()).await;

                    match result {
                        Ok(LoadedFile::Existing { text, sha }) => {
                            content.set(text.clone());
                            original_content.set(text);
                            file_sha.set(Some(sha));
                            is_new.set(false);
                            loading.set(false);
                        }
                        Ok(LoadedFile::New { template }) => {
                            content.set(template.clone());
                            original_content.set(String::new());
                            file_sha.set(None);
                            is_new.set(true);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                });
            }
            || ()
        });
    }

    let on_input = {
        let content = content.clone();
        let save_msg = save_msg.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(target.value());
            save_msg.set(None);
        })
    };

    let on_save = {
        let content = content.clone();
        let original_content = original_content.clone();
        let file_sha = file_sha.clone();
        let branch = branch.clone();
        let saving = saving.clone();
        let error = error.clone();
        let save_msg = save_msg.clone();
        let path = props.path.clone();
        let token = auth.token.clone();
        let is_new = is_new.clone();

        Callback::from(move |_: MouseEvent| {
            let content = content.clone();
            let original_content = original_content.clone();
            let file_sha = file_sha.clone();
            let branch = branch.clone();
            let saving = saving.clone();
            let error = error.clone();
            let save_msg = save_msg.clone();
            let path = path.clone();
            let is_new = is_new.clone();

            if let Some(token) = token.clone() {
                saving.set(true);
                error.set(None);
                save_msg.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);

                    // Ensure editor branch exists
                    let branch_name = match (*branch).clone() {
                        Some(b) => b,
                        None => match create_editor_branch(&client, &path).await {
                            Ok(name) => {
                                store_active_branch(&name);
                                branch.set(Some(name.clone()));
                                name
                            }
                            Err(e) => {
                                error.set(Some(e));
                                saving.set(false);
                                return;
                            }
                        },
                    };

                    let sha = if *is_new {
                        None
                    } else {
                        (*file_sha).clone()
                    };
                    let message = if *is_new {
                        format!("Create {path}")
                    } else {
                        format!("Update {path}")
                    };

                    match client
                        .create_or_update_file(
                            &path,
                            &content,
                            &message,
                            sha.as_deref(),
                            &branch_name,
                        )
                        .await
                    {
                        Ok(new_sha) => {
                            file_sha.set(Some(new_sha));
                            original_content.set((*content).clone());
                            is_new.set(false);
                            save_msg.set(Some("Saved".into()));
                            saving.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e));
                            saving.set(false);
                        }
                    }
                });
            }
        })
    };

    let on_delete = {
        let file_sha = file_sha.clone();
        let branch = branch.clone();
        let error = error.clone();
        let saving = saving.clone();
        let path = props.path.clone();
        let token = auth.token.clone();
        let navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let file_sha = file_sha.clone();
            let branch = branch.clone();
            let error = error.clone();
            let saving = saving.clone();
            let path = path.clone();
            let navigator = navigator.clone();

            if let Some(token) = token.clone() {
                let Some(sha) = (*file_sha).clone() else {
                    error.set(Some("Cannot delete: file has no SHA".into()));
                    return;
                };

                saving.set(true);
                error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);

                    // Ensure editor branch exists
                    let branch_name = match (*branch).clone() {
                        Some(b) => b,
                        None => match create_editor_branch(&client, &path).await {
                            Ok(name) => {
                                store_active_branch(&name);
                                branch.set(Some(name.clone()));
                                name
                            }
                            Err(e) => {
                                error.set(Some(e));
                                saving.set(false);
                                return;
                            }
                        },
                    };

                    let message = format!("Delete {path}");
                    match client
                        .delete_file(&path, &sha, &message, &branch_name)
                        .await
                    {
                        Ok(()) => {
                            saving.set(false);
                            navigator.push(&Route::Dashboard);
                        }
                        Err(e) => {
                            error.set(Some(e));
                            saving.set(false);
                        }
                    }
                });
            }
        })
    };

    let has_changes = *content != *original_content || *is_new;

    html! {
        <div class="editor-page">
            <div class="editor-header">
                <h2 class="editor-path">{&props.path}</h2>
                <div class="editor-meta">
                    if let Some(ref b) = *branch {
                        <span class="editor-branch">{format!("Branch: {b}")}</span>
                    }
                    if *is_new {
                        <span class="editor-badge new-badge">{"New file"}</span>
                    }
                </div>
            </div>

            if let Some(ref err) = *error {
                <p class="error">{err}</p>
            }

            if let Some(ref msg) = *save_msg {
                <p class="save-msg">{msg}</p>
            }

            if *loading {
                <p class="loading">{"Loading\u{2026}"}</p>
            } else {
                <div class="editor-toolbar">
                    <button
                        class="save-btn"
                        onclick={on_save}
                        disabled={*saving || !has_changes}
                    >
                        { if *saving { "Saving\u{2026}" } else { "Save" } }
                    </button>
                    if file_sha.is_some() {
                        <button
                            class="delete-btn"
                            onclick={on_delete}
                            disabled={*saving}
                        >
                            {"Delete"}
                        </button>
                    }
                </div>
                <textarea
                    class="editor-textarea"
                    value={(*content).clone()}
                    oninput={on_input}
                    spellcheck="false"
                />
            }
        </div>
    }
}

// ── File loading ────────────────────────────────────────────────

enum LoadedFile {
    Existing { text: String, sha: String },
    New { template: String },
}

/// Try to load a file: first from the editor branch (if any), then from source,
/// falling back to a new-file template if not found.
async fn load_file(
    client: &GitHubClient,
    path: &str,
    active_branch: Option<&str>,
) -> Result<LoadedFile, String> {
    // If there's an active editor branch, try it first
    if let Some(branch) = active_branch {
        match client.get_file(path, branch).await {
            Ok(file) => {
                let text = decode_github_content(&file.content.unwrap_or_default());
                return Ok(LoadedFile::Existing {
                    text,
                    sha: file.sha,
                });
            }
            Err(e) if e.contains("not found") => {
                // File not on editor branch yet, fall through to source
            }
            Err(e) => return Err(e),
        }
    }

    // Try the default branch
    match client.get_file(path, DEFAULT_BRANCH).await {
        Ok(file) => {
            let text = decode_github_content(&file.content.unwrap_or_default());
            Ok(LoadedFile::Existing {
                text,
                sha: file.sha,
            })
        }
        Err(e) if e.contains("not found") => Ok(LoadedFile::New {
            template: generate_template(path),
        }),
        Err(e) => Err(e),
    }
}

// ── Branch management ───────────────────────────────────────────

/// Create an editor branch from the source branch. Returns the branch name.
async fn create_editor_branch(client: &GitHubClient, path: &str) -> Result<String, String> {
    let source_sha = client.get_branch_sha(DEFAULT_BRANCH).await?;

    let today = js_sys::Date::new_0();
    let year = today.get_full_year();
    let month = today.get_month() + 1; // JS months are 0-indexed
    let day = today.get_date();
    let date_str = format!("{year}-{month:02}-{day:02}");

    let slug = slug_from_path(path);
    let branch_name = format!("editor/{date_str}-{slug}");

    client.create_branch(&branch_name, &source_sha).await?;
    Ok(branch_name)
}

fn get_active_branch() -> Option<String> {
    SessionStorage::get(BRANCH_KEY).ok()
}

fn store_active_branch(branch: &str) {
    let _ = SessionStorage::set(BRANCH_KEY, branch);
}

// ── Template generation ─────────────────────────────────────────

fn generate_template(path: &str) -> String {
    let today = js_sys::Date::new_0();
    let year = today.get_full_year();
    let month = today.get_month() + 1;
    let day = today.get_date();
    let date_str = format!("{year}-{month:02}-{day:02}");

    let slug = slug_from_path(path);
    let title = title_from_slug(&slug);

    format!(
        r#"+++
title = "{title}"
date = "{date_str}"
description = ""
draft = true
+++
"#
    )
}

fn slug_from_path(path: &str) -> String {
    let parts: Vec<&str> = path.trim_end_matches('/').split('/').collect();
    match parts.last() {
        Some(&"index.md") | Some(&"_index.md") => {
            // Use the parent directory name
            parts
                .get(parts.len().wrapping_sub(2))
                .unwrap_or(&"untitled")
                .to_string()
        }
        Some(last) => last.trim_end_matches(".md").to_string(),
        None => "untitled".to_string(),
    }
}

fn title_from_slug(slug: &str) -> String {
    let title = slug.replace('-', " ");
    let mut chars = title.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}
