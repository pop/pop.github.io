use std::rc::Rc;

use gloo_storage::{SessionStorage, Storage};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::components::dashboard::invalidate_cache;
use crate::models::github::{CompareResponse, DiffFile};
use crate::models::post::{parse_frontmatter, render_markdown};
use crate::routes::Route;
use crate::services::github::GitHubClient;

const DEFAULT_BRANCH: &str = "source";
const BRANCH_KEY: &str = "editor_branch";

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Edit,
    Preview,
    Split,
}

#[derive(Clone, PartialEq)]
enum CiState {
    Pending,
    Success,
    Failure(String), // html_url to the failing run
    None,            // no check runs exist
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
    let dragging = use_state(|| false);
    let file_input_ref = use_node_ref();
    let textarea_ref = use_node_ref();
    let save_btn_ref = use_node_ref();
    let has_unsaved = use_mut_ref(|| false);
    let rendered_html = use_state(String::new);
    let frontmatter_fields = use_state(Vec::<(String, String)>::new);
    let render_gen = use_mut_ref(|| 0u32);
    let show_diff = use_state(|| false);
    let diff_data = use_state(|| Option::<CompareResponse>::None);
    let diff_loading = use_state(|| false);
    let ci_status = use_state(|| CiState::None);

    // Auth-aware error setter: clears token on 401
    let set_error: Rc<dyn Fn(String)> = {
        let error = error.clone();
        let set_token = auth.set_token.clone();
        Rc::new(move |msg: String| {
            if msg.contains("Unauthorized") {
                set_token.emit(None);
            }
            error.set(Some(msg));
        })
    };

    let is_authenticated = auth.token.is_some();

    // Track unsaved changes for beforeunload
    {
        let has_unsaved = has_unsaved.clone();
        let content = content.clone();
        let original_content = original_content.clone();
        let is_new = is_new.clone();
        use_effect(move || {
            *has_unsaved.borrow_mut() = *content != *original_content || *is_new;
            || ()
        });
    }

    // Warn before closing tab with unsaved changes
    {
        let has_unsaved = has_unsaved.clone();
        use_effect_with((), move |_| {
            let listener = Closure::<dyn FnMut(web_sys::BeforeUnloadEvent)>::wrap(Box::new(
                move |e: web_sys::BeforeUnloadEvent| {
                    if *has_unsaved.borrow() {
                        e.prevent_default();
                        e.set_return_value("You have unsaved changes.");
                    }
                },
            ));

            let window = gloo_utils::window();
            let _ = window.add_event_listener_with_callback(
                "beforeunload",
                listener.as_ref().unchecked_ref(),
            );

            move || {
                let _ = window.remove_event_listener_with_callback(
                    "beforeunload",
                    listener.as_ref().unchecked_ref(),
                );
            }
        });
    }

    // Keyboard shortcuts (Ctrl+S / Cmd+S to save)
    {
        let save_btn_ref = save_btn_ref.clone();
        use_effect_with((), move |_| {
            let listener = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::wrap(Box::new(
                move |e: web_sys::KeyboardEvent| {
                    if (e.ctrl_key() || e.meta_key()) && e.key() == "s" {
                        e.prevent_default();
                        if let Some(btn) = save_btn_ref.cast::<HtmlElement>() {
                            btn.click();
                        }
                    }
                },
            ));

            let document = gloo_utils::document();
            let _ = document.add_event_listener_with_callback(
                "keydown",
                listener.as_ref().unchecked_ref(),
            );

            move || {
                let _ = document.remove_event_listener_with_callback(
                    "keydown",
                    listener.as_ref().unchecked_ref(),
                );
            }
        });
    }

    // Debounced markdown rendering for preview
    {
        let rendered_html = rendered_html.clone();
        let frontmatter_fields = frontmatter_fields.clone();
        let content_val = (*content).clone();
        let render_gen = render_gen.clone();
        let show_preview = *view_mode != ViewMode::Edit;

        use_effect_with((content_val, show_preview), move |(content_val, show_preview)| {
            if *show_preview {
                let gen = {
                    let mut g = render_gen.borrow_mut();
                    *g = g.wrapping_add(1);
                    *g
                };

                let content_val = content_val.clone();
                let rendered_html = rendered_html.clone();
                let frontmatter_fields = frontmatter_fields.clone();
                let render_gen = render_gen.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    sleep_ms(200).await;
                    if *render_gen.borrow() == gen {
                        frontmatter_fields.set(parse_frontmatter(&content_val));
                        rendered_html.set(render_markdown(&content_val));
                    }
                });
            }

            || ()
        });
    }

    // Syntax highlighting after preview render
    {
        let rendered_html_val = (*rendered_html).clone();
        use_effect_with(rendered_html_val, move |html| {
            if !html.is_empty() {
                highlight_code_blocks();
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
        let set_error = set_error.clone();
        let is_new = is_new.clone();
        let branch = branch.clone();
        let path = props.path.clone();
        let token = auth.token.clone();

        use_effect_with((path.clone(), token.clone()), move |_| {
            let client = match token {
                Some(t) => GitHubClient::new(t),
                None => GitHubClient::anonymous(),
            };
            let has_token = client.token.is_some();

            wasm_bindgen_futures::spawn_local(async move {
                let mut active_branch = (*branch).clone();

                // Verify stored branch still exists (only when authenticated)
                if has_token {
                    if let Some(ref branch_name) = active_branch {
                        match client.get_branch_sha(branch_name).await {
                            Ok(_) => {} // Branch exists
                            Err(e) if e.contains("not found") => {
                                clear_active_branch();
                                branch.set(None);
                                active_branch = None;
                            }
                            Err(e) => {
                                set_error(e);
                                loading.set(false);
                                return;
                            }
                        }
                    }
                } else {
                    // Anonymous users don't use editor branches
                    active_branch = None;
                }

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
                        set_error(e);
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // CI status: fetch on mount and poll every 15s while pending
    {
        let ci_status = ci_status.clone();
        let branch_val = (*branch).clone();
        let token = auth.token.clone();

        use_effect_with((branch_val.clone(), token.clone()), move |_| {
            let ci_status = ci_status.clone();
            let mut interval_handle: Option<i32> = None;

            if let (Some(branch_name), Some(token)) = (branch_val, token) {
                // Initial fetch
                {
                    let ci_status = ci_status.clone();
                    let branch_name = branch_name.clone();
                    let token = token.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        fetch_ci_status(&token, &branch_name, &ci_status).await;
                    });
                }

                // Poll every 15 seconds
                let cb_ci = ci_status.clone();
                let cb_branch = branch_name.clone();
                let cb_token = token.clone();

                let cb = Closure::<dyn FnMut()>::new(move || {
                    let ci_status = cb_ci.clone();
                    let branch_name = cb_branch.clone();
                    let token = cb_token.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        fetch_ci_status(&token, &branch_name, &ci_status).await;
                    });
                });

                let id = gloo_utils::window()
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        15_000,
                    )
                    .unwrap_or(0);
                cb.forget();
                interval_handle = Some(id);
            } else {
                ci_status.set(CiState::None);
            }

            move || {
                if let Some(id) = interval_handle {
                    let _ = gloo_utils::window().clear_interval_with_handle(id);
                }
            }
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
        let set_error = set_error.clone();
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
            let set_error = set_error.clone();
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
                                set_error(e);
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
                            invalidate_cache(parent_dir(&path));
                            save_msg.set(Some("Saved".into()));
                            saving.set(false);
                        }
                        Err(e) => {
                            set_error(e);
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
        let set_error = set_error.clone();
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
            let set_error = set_error.clone();
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
                                set_error(e);
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
                            invalidate_cache(parent_dir(&path));
                            saving.set(false);
                            navigator.push(&Route::Dashboard);
                        }
                        Err(e) => {
                            set_error(e);
                            saving.set(false);
                        }
                    }
                });
            }
        })
    };

    // Shared image upload callback (used by file input and drag-and-drop)
    let upload_image = {
        let content = content.clone();
        let branch = branch.clone();
        let uploading = uploading.clone();
        let error = error.clone();
        let set_error = set_error.clone();
        let save_msg = save_msg.clone();
        let path = props.path.clone();
        let token = auth.token.clone();
        let textarea_ref = textarea_ref.clone();

        Callback::from(move |file: web_sys::File| {
            // Validate image type
            let mime = file.type_();
            if !matches!(
                mime.as_str(),
                "image/png" | "image/jpeg" | "image/gif" | "image/webp" | "image/svg+xml"
            ) {
                error.set(Some(format!("Unsupported image type: {mime}")));
                return;
            }

            // Validate file size (10 MB max)
            const MAX_SIZE: f64 = 10.0 * 1024.0 * 1024.0;
            let size = file.size();
            if size > MAX_SIZE {
                error.set(Some(format!(
                    "Image too large ({:.1} MB). Maximum size is 10 MB.",
                    size / (1024.0 * 1024.0)
                )));
                return;
            }

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
            let set_error = set_error.clone();
            let save_msg = save_msg.clone();
            let path = path.clone();
            let textarea_ref = textarea_ref.clone();

            if let Some(token) = token.clone() {
                uploading.set(true);
                error.set(None);
                save_msg.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let bytes = match read_file_as_bytes(file).await {
                        Ok(b) => b,
                        Err(e) => {
                            set_error(e);
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
                                set_error(e);
                                uploading.set(false);
                                return;
                            }
                        },
                    };

                    // Check if image already exists (for overwrite)
                    let existing_sha = match client.get_file(&upload_path, &branch_name).await {
                        Ok(existing) => Some(existing.sha),
                        Err(_) => None,
                    };

                    let message = format!("Upload image {file_name}");
                    match client
                        .upload_binary_file(
                            &upload_path,
                            &bytes,
                            &message,
                            existing_sha.as_deref(),
                            &branch_name,
                        )
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
                            set_error(e);
                            uploading.set(false);
                        }
                    }
                });
            }
        })
    };

    let on_upload_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    let on_file_selected = {
        let upload_image = upload_image.clone();
        let file_input_ref = file_input_ref.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    upload_image.emit(file);
                }
            }
            // Reset file input so the same file can be re-selected
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.set_value("");
            }
        })
    };

    // Drag-and-drop image upload
    let on_dragover = {
        let dragging = dragging.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            dragging.set(true);
        })
    };

    let on_dragleave = {
        let dragging = dragging.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            dragging.set(false);
        })
    };

    let on_drop = {
        let dragging = dragging.clone();
        let upload_image = upload_image.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            dragging.set(false);

            if let Some(dt) = e.data_transfer() {
                if let Some(files) = dt.files() {
                    for i in 0..files.length() {
                        if let Some(file) = files.get(i) {
                            if file.type_().starts_with("image/") {
                                upload_image.emit(file);
                                break;
                            }
                        }
                    }
                }
            }
        })
    };

    // Publish: show diff first, then confirm
    let on_publish = {
        let branch = branch.clone();
        let error = error.clone();
        let set_error = set_error.clone();
        let token = auth.token.clone();
        let show_diff = show_diff.clone();
        let diff_data = diff_data.clone();
        let diff_loading = diff_loading.clone();

        Callback::from(move |_: MouseEvent| {
            let branch = branch.clone();
            let error = error.clone();
            let set_error = set_error.clone();
            let show_diff = show_diff.clone();
            let diff_data = diff_data.clone();
            let diff_loading = diff_loading.clone();

            let Some(branch_name) = (*branch).clone() else {
                error.set(Some("No active branch to publish".into()));
                return;
            };

            if let Some(token) = token.clone() {
                diff_loading.set(true);
                show_diff.set(false);
                error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    match client.compare_branches(DEFAULT_BRANCH, &branch_name).await {
                        Ok(data) => {
                            diff_data.set(Some(data));
                            show_diff.set(true);
                            diff_loading.set(false);
                        }
                        Err(e) => {
                            set_error(e);
                            diff_loading.set(false);
                        }
                    }
                });
            }
        })
    };

    let on_confirm_publish = {
        let branch = branch.clone();
        let saving = saving.clone();
        let error = error.clone();
        let set_error = set_error.clone();
        let save_msg = save_msg.clone();
        let token = auth.token.clone();
        let navigator = navigator.clone();
        let path = props.path.clone();
        let show_diff = show_diff.clone();

        Callback::from(move |_: MouseEvent| {
            let branch = branch.clone();
            let saving = saving.clone();
            let error = error.clone();
            let set_error = set_error.clone();
            let save_msg = save_msg.clone();
            let navigator = navigator.clone();
            let path = path.clone();
            let show_diff = show_diff.clone();

            let Some(branch_name) = (*branch).clone() else {
                error.set(Some("No active branch to publish".into()));
                return;
            };

            if let Some(token) = token.clone() {
                show_diff.set(false);
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
                            let _ = client.delete_branch(&branch_name).await;
                            clear_active_branch();
                            branch.set(None);
                            invalidate_cache(parent_dir(&path));
                            save_msg.set(Some("Published!".into()));
                            saving.set(false);
                            navigator.push(&Route::Dashboard);
                        }
                        Err(e) => {
                            set_error(e);
                            saving.set(false);
                        }
                    }
                });
            }
        })
    };

    let on_cancel_diff = {
        let show_diff = show_diff.clone();
        Callback::from(move |_: MouseEvent| {
            show_diff.set(false);
        })
    };

    let on_discard = {
        let branch = branch.clone();
        let saving = saving.clone();
        let error = error.clone();
        let set_error = set_error.clone();
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
            let set_error = set_error.clone();
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
                            crate::components::dashboard::invalidate_all_caches();
                            saving.set(false);
                            navigator.push(&Route::Dashboard);
                        }
                        Err(e) => {
                            set_error(e);
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
    let ci_blocks_publish = matches!(*ci_status, CiState::Pending | CiState::Failure(_));
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
                    if is_authenticated {
                        <button
                            ref={save_btn_ref.clone()}
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
                    } else {
                        <span class="login-hint">
                            <Link<Route> to={Route::Login}>{"Login to save"}</Link<Route>>
                        </span>
                    }
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
                if is_authenticated && branch.is_some() {
                    <div class="publish-bar">
                        <button
                            class="publish-btn"
                            onclick={on_publish}
                            disabled={*saving || *uploading || *diff_loading || has_changes || ci_blocks_publish}
                        >
                            { if *diff_loading { "Loading diff\u{2026}" } else { "Publish" } }
                        </button>
                        <button
                            class="discard-btn"
                            onclick={on_discard}
                            disabled={*saving || *uploading}
                        >
                            {"Discard"}
                        </button>
                        {render_ci_indicator(&ci_status)}
                        if has_changes {
                            <span class="publish-hint">{"Save changes before publishing"}</span>
                        }
                    </div>
                    if *show_diff {
                        if let Some(ref data) = *diff_data {
                            {render_diff_panel(data, on_confirm_publish, on_cancel_diff)}
                        }
                    }
                }
                <div
                    class={classes!(
                        "editor-container",
                        is_split.then_some("split"),
                        (*dragging).then_some("drag-over"),
                    )}
                    ondragover={on_dragover}
                    ondragleave={on_dragleave}
                    ondrop={on_drop}
                >
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
                            if rendered_html.is_empty() {
                                <p class="loading">{"Rendering\u{2026}"}</p>
                            } else {
                                if !frontmatter_fields.is_empty() {
                                    <table class="frontmatter-table">
                                        { for frontmatter_fields.iter().map(|(key, value)| html! {
                                            <tr>
                                                <td class="fm-key">{key}</td>
                                                <td class="fm-value">{value}</td>
                                            </tr>
                                        }) }
                                    </table>
                                }
                                {Html::from_html_unchecked(AttrValue::from((*rendered_html).clone()))}
                            }
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
                let text = file.content.unwrap_or_default();
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
            let text = file.content.unwrap_or_default();
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

// ── Debounce / highlighting helpers ─────────────────────────────

/// Async sleep using setTimeout.
async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let _ = gloo_utils::window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Call highlight.js on all unhighlighted code blocks in the DOM.
fn highlight_code_blocks() {
    let _ = js_sys::eval(
        "if(typeof hljs!=='undefined'){document.querySelectorAll('pre code:not(.hljs)').forEach(el=>hljs.highlightElement(el));}",
    );
}

// ── CI status ───────────────────────────────────────────────────

async fn fetch_ci_status(
    token: &str,
    branch: &str,
    ci_status: &UseStateHandle<CiState>,
) {
    // Don't re-fetch if already in a terminal state
    let current = (*ci_status).clone();
    if matches!(&*current, CiState::Success | CiState::Failure(_)) {
        return;
    }

    let client = GitHubClient::new(token.to_string());
    match client.get_check_runs(branch).await {
        Ok(resp) => {
            if resp.check_runs.is_empty() {
                ci_status.set(CiState::None);
                return;
            }
            let any_pending = resp.check_runs.iter().any(|r| r.status != "completed");
            if any_pending {
                ci_status.set(CiState::Pending);
                return;
            }
            let failure = resp.check_runs.iter().find(|r| {
                r.conclusion.as_deref() == Some("failure")
            });
            if let Some(failed) = failure {
                ci_status.set(CiState::Failure(failed.html_url.clone()));
            } else {
                ci_status.set(CiState::Success);
            }
        }
        Err(_) => {
            // Silently ignore CI fetch errors (don't block the user)
        }
    }
}

fn render_ci_indicator(ci_status: &UseStateHandle<CiState>) -> Html {
    match &**ci_status {
        CiState::None => html! {},
        CiState::Pending => html! {
            <span class="ci-status ci-pending" title="CI check running">
                {"\u{23F3} CI pending\u{2026}"}
            </span>
        },
        CiState::Success => html! {
            <span class="ci-status ci-success" title="CI check passed">
                {"\u{2713} CI passed"}
            </span>
        },
        CiState::Failure(url) => html! {
            <span class="ci-status ci-failure">
                {"\u{2717} CI failed "}
                <a href={url.clone()} target="_blank" rel="noopener" class="ci-link">
                    {"(view)"}
                </a>
            </span>
        },
    }
}

// ── Diff rendering ──────────────────────────────────────────────

fn render_diff_panel(
    data: &CompareResponse,
    on_confirm: Callback<MouseEvent>,
    on_cancel: Callback<MouseEvent>,
) -> Html {
    let total_additions: u32 = data.files.iter().map(|f| f.additions).sum();
    let total_deletions: u32 = data.files.iter().map(|f| f.deletions).sum();

    if data.files.is_empty() {
        return html! {
            <div class="diff-panel">
                <p class="diff-empty">{"No changes to publish."}</p>
                <div class="diff-actions">
                    <button class="discard-btn" onclick={on_cancel}>{"Close"}</button>
                </div>
            </div>
        };
    }

    html! {
        <div class="diff-panel">
            <div class="diff-header">
                <h3>{"Changes to publish"}</h3>
                <p class="diff-summary">
                    {format!("{} file{} changed, ",
                        data.files.len(),
                        if data.files.len() == 1 { "" } else { "s" }
                    )}
                    <span class="diff-additions">{format!("+{total_additions}")}</span>
                    {", "}
                    <span class="diff-deletions">{format!("-{total_deletions}")}</span>
                </p>
            </div>
            <div class="diff-files">
                { for data.files.iter().map(render_diff_file) }
            </div>
            <div class="diff-actions">
                <button class="publish-btn" onclick={on_confirm}>
                    {"Confirm Publish"}
                </button>
                <button class="discard-btn" onclick={on_cancel}>
                    {"Cancel"}
                </button>
            </div>
        </div>
    }
}

fn render_diff_file(file: &DiffFile) -> Html {
    let status_class = match file.status.as_str() {
        "added" => "added",
        "removed" => "removed",
        "renamed" => "renamed",
        _ => "modified",
    };
    let status_letter = match file.status.as_str() {
        "added" => "A",
        "removed" => "D",
        "renamed" => "R",
        _ => "M",
    };

    html! {
        <div class="diff-file">
            <div class="diff-file-header">
                <span class={classes!("diff-file-status", status_class)}>
                    {status_letter}
                </span>
                <span class="diff-file-name">{&file.filename}</span>
                <span class="diff-file-stats">
                    <span class="diff-additions">{format!("+{}", file.additions)}</span>
                    {" "}
                    <span class="diff-deletions">{format!("-{}", file.deletions)}</span>
                </span>
            </div>
            if let Some(ref patch) = file.patch {
                <pre class="diff-patch">
                    { for patch.lines().map(|line| {
                        let class = if line.starts_with('+') {
                            "diff-line-add"
                        } else if line.starts_with('-') {
                            "diff-line-del"
                        } else if line.starts_with("@@") {
                            "diff-line-hunk"
                        } else {
                            "diff-line-ctx"
                        };
                        html! { <div class={classes!("diff-line", class)}>{line}</div> }
                    }) }
                </pre>
            }
        </div>
    }
}
