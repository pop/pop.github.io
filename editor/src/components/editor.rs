use gloo_storage::{SessionStorage, Storage};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::models::post::render_markdown;
use crate::routes::Route;
use crate::services::github::{decode_github_content, GitHubClient};

const DEFAULT_BRANCH: &str = "source";
const BRANCH_KEY: &str = "editor_branch";

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Edit,
    Preview,
    Split,
}

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
    let uploading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let save_msg = use_state(|| Option::<String>::None);
    let is_new = use_state(|| false);
    let view_mode = use_state(|| ViewMode::Edit);
    let file_input_ref = use_node_ref();
    let textarea_ref = use_node_ref();

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
            let window = gloo_utils::window();
            if !window
                .confirm_with_message("Delete this file? This cannot be undone.")
                .unwrap_or(false)
            {
                return;
            }

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

    // Image upload callbacks
    let on_upload_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    let on_file_selected = {
        let content = content.clone();
        let branch = branch.clone();
        let uploading = uploading.clone();
        let error = error.clone();
        let save_msg = save_msg.clone();
        let path = props.path.clone();
        let token = auth.token.clone();
        let textarea_ref = textarea_ref.clone();
        let file_input_ref = file_input_ref.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let Some(files) = input.files() else { return };
            let Some(file) = files.get(0) else { return };

            let file_name = sanitize_filename(&file.name());
            let upload_dir = parent_dir(&path);
            let upload_path = if upload_dir.is_empty() {
                file_name.clone()
            } else {
                format!("{upload_dir}/{file_name}")
            };

            let content = content.clone();
            let branch = branch.clone();
            let uploading = uploading.clone();
            let error = error.clone();
            let save_msg = save_msg.clone();
            let path = path.clone();
            let textarea_ref = textarea_ref.clone();
            let file_input_ref = file_input_ref.clone();

            if let Some(token) = token.clone() {
                uploading.set(true);
                error.set(None);
                save_msg.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let bytes = match read_file_as_bytes(file).await {
                        Ok(b) => b,
                        Err(e) => {
                            error.set(Some(e));
                            uploading.set(false);
                            return;
                        }
                    };

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
                                uploading.set(false);
                                return;
                            }
                        },
                    };

                    let message = format!("Upload image {file_name}");
                    match client
                        .upload_binary_file(&upload_path, &bytes, &message, &branch_name)
                        .await
                    {
                        Ok(_sha) => {
                            // Insert markdown image reference at cursor position
                            let md_ref = format!("![{file_name}]({file_name})");
                            let current = (*content).clone();

                            let new_content =
                                if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                                    if let Ok(Some(pos)) = textarea.selection_start() {
                                        let insert_at =
                                            char_pos_to_byte_offset(&current, pos as usize);
                                        let (before, after) = current.split_at(insert_at);
                                        format!("{before}{md_ref}{after}")
                                    } else {
                                        format!("{current}\n{md_ref}")
                                    }
                                } else {
                                    format!("{current}\n{md_ref}")
                                };

                            content.set(new_content);
                            save_msg.set(Some(format!("Uploaded {file_name}")));
                            uploading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e));
                            uploading.set(false);
                        }
                    }

                    // Reset file input so the same file can be re-selected
                    if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                        input.set_value("");
                    }
                });
            }
        })
    };

    // Publish & discard callbacks
    let on_publish = {
        let branch = branch.clone();
        let saving = saving.clone();
        let error = error.clone();
        let save_msg = save_msg.clone();
        let token = auth.token.clone();
        let navigator = navigator.clone();
        let path = props.path.clone();

        Callback::from(move |_: MouseEvent| {
            let branch = branch.clone();
            let saving = saving.clone();
            let error = error.clone();
            let save_msg = save_msg.clone();
            let navigator = navigator.clone();
            let path = path.clone();

            let Some(branch_name) = (*branch).clone() else {
                error.set(Some("No active branch to publish".into()));
                return;
            };

            if let Some(token) = token.clone() {
                saving.set(true);
                error.set(None);
                save_msg.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    let slug = slug_from_path(&path);
                    let message = format!("Publish: {slug}");

                    match client
                        .merge_branch(&branch_name, DEFAULT_BRANCH, &message)
                        .await
                    {
                        Ok(()) => {
                            // Clean up: delete editor branch and clear state
                            let _ = client.delete_branch(&branch_name).await;
                            clear_active_branch();
                            branch.set(None);
                            save_msg.set(Some("Published!".into()));
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

    let on_discard = {
        let branch = branch.clone();
        let saving = saving.clone();
        let error = error.clone();
        let token = auth.token.clone();
        let navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let window = gloo_utils::window();
            if !window
                .confirm_with_message(
                    "Discard all changes? This will delete the editor branch.",
                )
                .unwrap_or(false)
            {
                return;
            }

            let branch = branch.clone();
            let saving = saving.clone();
            let error = error.clone();
            let navigator = navigator.clone();

            let Some(branch_name) = (*branch).clone() else {
                error.set(Some("No active branch to discard".into()));
                return;
            };

            if let Some(token) = token.clone() {
                saving.set(true);
                error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);

                    match client.delete_branch(&branch_name).await {
                        Ok(()) => {
                            clear_active_branch();
                            branch.set(None);
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

    // View mode toggle callbacks
    let set_edit = {
        let view_mode = view_mode.clone();
        Callback::from(move |_: MouseEvent| view_mode.set(ViewMode::Edit))
    };
    let set_preview = {
        let view_mode = view_mode.clone();
        Callback::from(move |_: MouseEvent| view_mode.set(ViewMode::Preview))
    };
    let set_split = {
        let view_mode = view_mode.clone();
        Callback::from(move |_: MouseEvent| view_mode.set(ViewMode::Split))
    };

    let has_changes = *content != *original_content || *is_new;
    let show_editor = *view_mode != ViewMode::Preview;
    let show_preview = *view_mode != ViewMode::Edit;
    let is_split = *view_mode == ViewMode::Split;

    html! {
        <div class={classes!("editor-page", is_split.then_some("editor-page-wide"))}>
            <div class="editor-header">
                <div class="editor-nav">
                    <Link<Route> to={Route::Dashboard} classes="back-link">
                        {"\u{2190} Dashboard"}
                    </Link<Route>>
                </div>
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
                            disabled={*saving || *uploading}
                        >
                            {"Delete"}
                        </button>
                    }
                    <button
                        class="upload-btn"
                        onclick={on_upload_click}
                        disabled={*saving || *uploading}
                    >
                        { if *uploading { "Uploading\u{2026}" } else { "Upload Image" } }
                    </button>
                    <input
                        ref={file_input_ref.clone()}
                        type="file"
                        accept="image/*"
                        class="hidden-file-input"
                        onchange={on_file_selected}
                    />
                    <div class="view-toggle">
                        <button
                            class={classes!("toggle-btn", (*view_mode == ViewMode::Edit).then_some("active"))}
                            onclick={set_edit}
                        >{"Edit"}</button>
                        <button
                            class={classes!("toggle-btn", (*view_mode == ViewMode::Preview).then_some("active"))}
                            onclick={set_preview}
                        >{"Preview"}</button>
                        <button
                            class={classes!("toggle-btn", (*view_mode == ViewMode::Split).then_some("active"))}
                            onclick={set_split}
                        >{"Split"}</button>
                    </div>
                </div>
                if branch.is_some() {
                    <div class="publish-bar">
                        <button
                            class="publish-btn"
                            onclick={on_publish}
                            disabled={*saving || *uploading || has_changes}
                        >
                            {"Publish"}
                        </button>
                        <button
                            class="discard-btn"
                            onclick={on_discard}
                            disabled={*saving || *uploading}
                        >
                            {"Discard"}
                        </button>
                        if has_changes {
                            <span class="publish-hint">{"Save changes before publishing"}</span>
                        }
                    </div>
                }
                <div class={classes!("editor-container", is_split.then_some("split"))}>
                    if show_editor {
                        <textarea
                            ref={textarea_ref.clone()}
                            class="editor-textarea"
                            value={(*content).clone()}
                            oninput={on_input}
                            spellcheck="false"
                        />
                    }
                    if show_preview {
                        <div class="preview-pane markdown-body">
                            {Html::from_html_unchecked(AttrValue::from(render_markdown(&content)))}
                        </div>
                    }
                </div>
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

fn clear_active_branch() {
    SessionStorage::delete(BRANCH_KEY);
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

// ── Image upload helpers ─────────────────────────────────────────

/// Read a browser File as bytes using the FileReader API.
async fn read_file_as_bytes(file: web_sys::File) -> Result<Vec<u8>, String> {
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let reader = web_sys::FileReader::new().unwrap();
        let reader2 = reader.clone();
        let resolve = resolve.clone();
        let reject = reject.clone();

        let onload = Closure::once(move || {
            let result = reader2.result().unwrap();
            let _ = resolve.call1(&JsValue::NULL, &result);
        });

        let onerror = Closure::once(move || {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("Failed to read file"));
        });

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onload.forget();
        onerror.forget();

        let _ = reader.read_as_array_buffer(&file);
    });

    let result = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| "Failed to read file".to_string())?;

    let array = js_sys::Uint8Array::new(&result);
    Ok(array.to_vec())
}

/// Get the parent directory of a file path.
fn parent_dir(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/// Sanitize a filename for use in a URL path (lowercase, no spaces).
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c == ' ' { '-' } else { c })
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect::<String>()
        .to_lowercase()
}

/// Convert a character position (from JS selectionStart) to a byte offset in a UTF-8 string.
fn char_pos_to_byte_offset(s: &str, char_pos: usize) -> usize {
    s.char_indices()
        .nth(char_pos)
        .map_or(s.len(), |(byte_idx, _)| byte_idx)
}
