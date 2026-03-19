use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AuthContext;
use crate::components::dashboard::invalidate_cache;
use crate::models::github::CommitSummary;
use crate::models::post::{parse_frontmatter, post_dir, render_markdown};
use crate::routes::Route;
use crate::services::github::GitHubClient;

const DEFAULT_BRANCH: &str = "source";

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
    let show_history = use_state(|| false);
    let history_commits = use_state(|| Vec::<CommitSummary>::new());
    let history_loading = use_state(|| false);
    let history_error = use_state(|| Option::<String>::None);
    let reverting = use_state(|| false);
    let history_preview_pre_sha = use_state(|| Option::<String>::None);
    let pending_revert_sha = use_state(|| Option::<String>::None);

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
            let _ = document
                .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref());

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
        let token = auth.token.clone();
        let path = props.path.clone();
        let active_branch = auth.active_branch.clone();

        use_effect_with(
            (content_val, show_preview),
            move |(content_val, show_preview)| {
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
                    let token = token.clone();
                    let path = path.clone();
                    let active_branch = active_branch.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        sleep_ms(200).await;
                        if *render_gen.borrow() == gen {
                            frontmatter_fields.set(parse_frontmatter(&content_val));
                            let raw_html = render_markdown(&content_val);
                            let client = match token {
                                Some(t) => GitHubClient::new(t),
                                None => GitHubClient::anonymous(),
                            };
                            let branch = active_branch
                                .as_deref()
                                .unwrap_or(DEFAULT_BRANCH)
                                .to_string();
                            let resolved = client
                                .resolve_images_in_html(&raw_html, &path, &branch)
                                .await;
                            rendered_html.set(resolved);
                        }
                    });
                }

                || ()
            },
        );
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
        let set_active_branch = auth.set_active_branch.clone();
        let initial_branch = auth.active_branch.clone();
        let path = props.path.clone();
        let token = auth.token.clone();

        use_effect_with((path.clone(), token.clone()), move |_| {
            let client = match token {
                Some(t) => GitHubClient::new(t),
                None => GitHubClient::anonymous(),
            };
            let has_token = client.token.is_some();

            wasm_bindgen_futures::spawn_local(async move {
                let mut active_branch = initial_branch;

                // When authenticated with an active branch, verify it exists and load
                // the file in a single batched GraphQL query (saves one round-trip).
                if has_token {
                    if let Some(ref branch_name) = active_branch.clone() {
                        match client.get_branch_sha_and_file(branch_name, &path).await {
                            Err(e) => {
                                set_error(e);
                                loading.set(false);
                                return;
                            }
                            Ok(None) => {
                                // Branch no longer exists — fall back to source
                                set_active_branch.emit(None);
                                active_branch = None;
                            }
                            Ok(Some((_, Some(file)))) => {
                                // File found on active branch — done
                                let text = file.content.unwrap_or_default();
                                content.set(text.clone());
                                original_content.set(text);
                                file_sha.set(Some(file.sha));
                                is_new.set(false);
                                loading.set(false);
                                return;
                            }
                            Ok(Some((_, None))) => {
                                // Branch exists but file not on it — fall back to source
                                active_branch = None;
                            }
                        }
                    }
                } else {
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

    let on_input = {
        let content = content.clone();
        let save_msg = save_msg.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(target.value());
            save_msg.set(None);
        })
    };

    // Phase 20: formatting toolbar callbacks
    macro_rules! make_format_cb {
        ($prefix:expr, $suffix:expr, $placeholder:expr) => {{
            let content = content.clone();
            let textarea_ref = textarea_ref.clone();
            Callback::from(move |_: MouseEvent| {
                let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() else {
                    return;
                };
                let current = (*content).clone();
                let new_content =
                    apply_format_to_content(&textarea, &current, $prefix, $suffix, $placeholder);
                content.set(new_content);
                let _ = textarea.focus();
            })
        }};
    }
    let on_fmt_bold = make_format_cb!("**", "**", "bold text");
    let on_fmt_italic = make_format_cb!("*", "*", "italic text");
    let on_fmt_strike = make_format_cb!("~~", "~~", "strikethrough text");
    let on_fmt_code_inline = make_format_cb!("`", "`", "code");
    let on_fmt_code_block = make_format_cb!("```\n", "\n```", "code");

    let on_save = {
        let content = content.clone();
        let original_content = original_content.clone();
        let file_sha = file_sha.clone();
        let branch_opt = auth.active_branch.clone();
        let set_active_branch = auth.set_active_branch.clone();
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
            let branch_opt = branch_opt.clone();
            let set_active_branch = set_active_branch.clone();
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

                    let branch_name = match branch_opt {
                        Some(b) => b,
                        None => match create_editor_branch(&client, &path).await {
                            Ok(name) => {
                                set_active_branch.emit(Some(name.clone()));
                                name
                            }
                            Err(e) => {
                                set_error(e);
                                saving.set(false);
                                return;
                            }
                        },
                    };

                    let sha = if *is_new { None } else { (*file_sha).clone() };
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
                            invalidate_cache(post_dir(&path));
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
        let branch_opt = auth.active_branch.clone();
        let set_active_branch = auth.set_active_branch.clone();
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
            let branch_opt = branch_opt.clone();
            let set_active_branch = set_active_branch.clone();
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

                    let branch_name = match branch_opt {
                        Some(b) => b,
                        None => match create_editor_branch(&client, &path).await {
                            Ok(name) => {
                                set_active_branch.emit(Some(name.clone()));
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
                            invalidate_cache(post_dir(&path));
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
        let branch_opt = auth.active_branch.clone();
        let set_active_branch = auth.set_active_branch.clone();
        let uploading = uploading.clone();
        let error = error.clone();
        let set_error = set_error.clone();
        let save_msg = save_msg.clone();
        let path = props.path.clone();
        let token = auth.token.clone();
        let textarea_ref = textarea_ref.clone();

        Callback::from(move |file: web_sys::File| {
            let mime = file.type_();
            if !matches!(
                mime.as_str(),
                "image/png" | "image/jpeg" | "image/gif" | "image/webp" | "image/svg+xml"
            ) {
                error.set(Some(format!("Unsupported image type: {mime}")));
                return;
            }

            const MAX_SIZE: f64 = 10.0 * 1024.0 * 1024.0;
            let size = file.size();
            if size > MAX_SIZE {
                error.set(Some(format!(
                    "Image too large ({:.1} MB). Maximum size is 10 MB.",
                    size / (1024.0 * 1024.0)
                )));
                return;
            }

            let file_name = crate::utils::sanitize_filename(&file.name());
            let upload_dir = post_dir(&path);
            let upload_path = if upload_dir.is_empty() {
                file_name.clone()
            } else {
                format!("{upload_dir}/{file_name}")
            };

            let content = content.clone();
            let branch_opt = branch_opt.clone();
            let set_active_branch = set_active_branch.clone();
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

                    let branch_name = match branch_opt {
                        Some(b) => b,
                        None => match create_editor_branch(&client, &path).await {
                            Ok(name) => {
                                set_active_branch.emit(Some(name.clone()));
                                name
                            }
                            Err(e) => {
                                set_error(e);
                                uploading.set(false);
                                return;
                            }
                        },
                    };

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
                            let md_ref = format!("![{file_name}]({file_name})");
                            let current = (*content).clone();

                            let new_content = if let Some(textarea) =
                                textarea_ref.cast::<HtmlTextAreaElement>()
                            {
                                if let Ok(Some(pos)) = textarea.selection_start() {
                                    let insert_at = crate::utils::char_pos_to_byte_offset(
                                        &current,
                                        pos as usize,
                                    );
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
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.set_value("");
            }
        })
    };

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

    let toggle_history = {
        let show_history = show_history.clone();
        Callback::from(move |_: MouseEvent| {
            show_history.set(!*show_history);
        })
    };

    let on_history_select = {
        let pending_revert_sha = pending_revert_sha.clone();
        Callback::from(move |sha: String| {
            pending_revert_sha.set(Some(sha));
        })
    };

    let on_cancel_pending_revert = {
        let pending_revert_sha = pending_revert_sha.clone();
        Callback::from(move |_: MouseEvent| {
            pending_revert_sha.set(None);
        })
    };

    let on_discard_then_revert = {
        let content = content.clone();
        let original_content = original_content.clone();
        let file_sha = file_sha.clone();
        let is_new = is_new.clone();
        let token = auth.token.clone();
        let path = props.path.clone();
        let active_branch = auth.active_branch.clone();
        let set_error = set_error.clone();
        Callback::from(move |_: MouseEvent| {
            if let (Some(token), Some(branch)) = (token.clone(), active_branch.clone()) {
                let content = content.clone();
                let original_content = original_content.clone();
                let file_sha = file_sha.clone();
                let is_new = is_new.clone();
                let set_error = set_error.clone();
                let path = path.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    match client.get_file(&path, &branch).await {
                        Ok(f) => {
                            let text = f.content.unwrap_or_default();
                            content.set(text.clone());
                            original_content.set(text);
                            file_sha.set(Some(f.sha));
                            is_new.set(false);
                        }
                        Err(e) => set_error(e),
                    }
                });
            }
        })
    };

    let on_save_then_revert = {
        let save_btn_ref = save_btn_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(btn) = save_btn_ref.cast::<HtmlElement>() {
                btn.click();
            }
        })
    };

    let on_confirm_revert = {
        let history_preview_pre_sha = history_preview_pre_sha.clone();
        let save_msg = save_msg.clone();
        Callback::from(move |_: MouseEvent| {
            history_preview_pre_sha.set(None);
            save_msg.set(Some("Version restored".into()));
        })
    };

    let on_cancel_revert = {
        let history_preview_pre_sha = history_preview_pre_sha.clone();
        let reverting = reverting.clone();
        let content = content.clone();
        let original_content = original_content.clone();
        let file_sha = file_sha.clone();
        let set_error = set_error.clone();
        let token = auth.token.clone();
        let path = props.path.clone();
        let active_branch = auth.active_branch.clone();
        Callback::from(move |_: MouseEvent| {
            let Some(pre_sha) = (*history_preview_pre_sha).clone() else {
                return;
            };
            if let (Some(token), Some(branch)) = (token.clone(), active_branch.clone()) {
                let reverting = reverting.clone();
                let content = content.clone();
                let original_content = original_content.clone();
                let file_sha = file_sha.clone();
                let set_error = set_error.clone();
                let history_preview_pre_sha = history_preview_pre_sha.clone();
                let path = path.clone();
                reverting.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    let client = GitHubClient::new(token);
                    match client
                        .revert_directory_to_commit(&path, &pre_sha, &branch)
                        .await
                    {
                        Ok(new_file_sha) => {
                            if let Ok(f) = client.get_file(&path, &branch).await {
                                let text = f.content.unwrap_or_default();
                                content.set(text.clone());
                                original_content.set(text);
                                file_sha.set(Some(new_file_sha));
                            }
                            history_preview_pre_sha.set(None);
                            reverting.set(false);
                        }
                        Err(e) => {
                            set_error(e);
                            reverting.set(false);
                        }
                    }
                });
            }
        })
    };

    // Revert effect: fires when pending_revert_sha is Some and content is clean
    {
        let pending_revert_sha_state = pending_revert_sha.clone();
        let content = content.clone();
        let original_content = original_content.clone();
        let is_new = is_new.clone();
        let reverting = reverting.clone();
        let history_preview_pre_sha = history_preview_pre_sha.clone();
        let history_commits = history_commits.clone();
        let file_sha = file_sha.clone();
        let view_mode = view_mode.clone();
        let set_error = set_error.clone();
        let token = auth.token.clone();
        let path = props.path.clone();
        let active_branch = auth.active_branch.clone();

        use_effect_with(
            ((*pending_revert_sha).clone(), (*original_content).clone()),
            move |(pending_sha, _)| {
                let ready = pending_sha.is_some()
                    && !(*content != *original_content || *is_new)
                    && token.is_some()
                    && active_branch.is_some();

                if ready {
                    let sha = pending_sha.clone().unwrap();
                    let token = token.unwrap();
                    let branch = active_branch.unwrap();
                    let pending_revert_sha = pending_revert_sha_state.clone();
                    let content = content.clone();
                    let original_content = original_content.clone();
                    let file_sha = file_sha.clone();
                    let reverting = reverting.clone();
                    let history_preview_pre_sha = history_preview_pre_sha.clone();
                    let history_commits = history_commits.clone();
                    let view_mode = view_mode.clone();
                    let set_error = set_error.clone();
                    let path = path.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        let client = GitHubClient::new(token);

                        let pre_sha = match client.get_branch_sha(&branch).await {
                            Ok(s) => s,
                            Err(e) => {
                                set_error(e);
                                return;
                            }
                        };

                        reverting.set(true);

                        match client
                            .revert_directory_to_commit(&path, &sha, &branch)
                            .await
                        {
                            Ok(new_file_sha) => {
                                match client.get_file(&path, &branch).await {
                                    Ok(f) => {
                                        let text = f.content.unwrap_or_default();
                                        content.set(text.clone());
                                        original_content.set(text);
                                        file_sha.set(Some(new_file_sha));
                                    }
                                    Err(e) => {
                                        set_error(e);
                                        reverting.set(false);
                                        return;
                                    }
                                }
                                history_preview_pre_sha.set(Some(pre_sha));
                                pending_revert_sha.set(None);
                                view_mode.set(ViewMode::Split);
                                history_commits.set(vec![]);
                                reverting.set(false);
                            }
                            Err(e) => {
                                set_error(e);
                                pending_revert_sha.set(None);
                                reverting.set(false);
                            }
                        }
                    });
                }

                || ()
            },
        );
    }

    // Fetch history when panel is opened for the first time
    {
        let history_commits = history_commits.clone();
        let history_loading = history_loading.clone();
        let history_error = history_error.clone();
        let token = auth.token.clone();
        let path = props.path.clone();
        let active_branch = auth.active_branch.clone();
        let show_history_val = *show_history;

        use_effect_with(
            (show_history_val, auth.active_branch.clone()),
            move |_| {
                if show_history_val && history_commits.is_empty() {
                    if let (Some(token), Some(branch)) = (token, active_branch) {
                        history_loading.set(true);
                        history_error.set(None);
                        wasm_bindgen_futures::spawn_local(async move {
                            let client = GitHubClient::new(token);
                            match client.list_commits_for_path(&path, &branch).await {
                                Ok(commits) => {
                                    history_commits.set(commits);
                                    history_loading.set(false);
                                }
                                Err(e) => {
                                    history_error.set(Some(e));
                                    history_loading.set(false);
                                }
                            }
                        });
                    }
                }
                || ()
            },
        );
    }

    let has_changes = *content != *original_content || *is_new;
    let show_editor = *view_mode != ViewMode::Preview;
    let show_preview = *view_mode != ViewMode::Edit;
    let is_split = *view_mode == ViewMode::Split;

    html! {
        <div class={classes!("editor-page", is_split.then_some("editor-page-wide"))}>
            <div class="editor-header">
                <div class="editor-nav">
                    <Link<Route> to={Route::Dashboard} classes="back-link">
                        {"\u{2190} Back"}
                    </Link<Route>>
                </div>
                <h2 class="editor-path">{&props.path}</h2>
                <div class="editor-meta">
                    if let Some(ref b) = auth.active_branch {
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
                            disabled={*saving || !has_changes || history_preview_pre_sha.is_some()}
                        >
                            { if *saving { "Saving\u{2026}" } else { "Save" } }
                        </button>
                        if auth.active_branch.is_some() {
                            if file_sha.is_some() {
                                <button
                                    class="history-toggle-btn"
                                    onclick={toggle_history.clone()}
                                    disabled={*saving || *history_loading || history_preview_pre_sha.is_some()}
                                >
                                    { if *show_history { "Hide history" } else { "History" } }
                                </button>
                            } else {
                                <button
                                    class="history-toggle-btn"
                                    disabled=true
                                    title="Save the post at least once to view history"
                                >
                                    {"History"}
                                </button>
                            }
                        }
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
                    <div class="format-buttons">
                        <button class="format-btn" onclick={on_fmt_bold} title="Bold">{"B"}</button>
                        <button class="format-btn format-btn-italic" onclick={on_fmt_italic} title="Italic">{"I"}</button>
                        <button class="format-btn format-btn-strike" onclick={on_fmt_strike} title="Strikethrough">{"S"}</button>
                        <button class="format-btn" onclick={on_fmt_code_inline} title="Inline code">{"`"}</button>
                        <button class="format-btn" onclick={on_fmt_code_block} title="Code block">{"```"}</button>
                    </div>
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
                if *show_history {
                    {render_history_panel(
                        &history_commits,
                        *history_loading,
                        &history_error,
                        on_history_select.clone(),
                    )}
                    if pending_revert_sha.is_some() && has_changes {
                        <div class="revert-gate-banner">
                            <p>{"You have unsaved changes. Save or discard them before reverting."}</p>
                            <div class="revert-gate-actions">
                                <button
                                    class="save-btn"
                                    onclick={on_save_then_revert.clone()}
                                    disabled={*saving || *reverting}
                                >
                                    { if *saving { "Saving\u{2026}" } else { "Save and revert" } }
                                </button>
                                <button
                                    class="discard-btn"
                                    onclick={on_discard_then_revert.clone()}
                                    disabled={*saving || *reverting}
                                >
                                    {"Discard and revert"}
                                </button>
                                <button onclick={on_cancel_pending_revert.clone()}>{"Cancel"}</button>
                            </div>
                        </div>
                    }
                }
                if history_preview_pre_sha.is_some() {
                    <div class="revert-preview-banner">
                        <span class="revert-preview-msg">
                            {"Previewing restored version \u{2014} confirm or cancel."}
                        </span>
                        <div class="revert-preview-actions">
                            <button
                                class="confirm-revert-btn"
                                onclick={on_confirm_revert.clone()}
                                disabled={*reverting}
                            >
                                {"Confirm restore"}
                            </button>
                            <button
                                class="cancel-revert-btn"
                                onclick={on_cancel_revert.clone()}
                                disabled={*reverting}
                            >
                                { if *reverting { "Reverting\u{2026}" } else { "Cancel restore" } }
                            </button>
                        </div>
                    </div>
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
                            spellcheck="true"
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

async fn load_file(
    client: &GitHubClient,
    path: &str,
    active_branch: Option<&str>,
) -> Result<LoadedFile, String> {
    if let Some(branch) = active_branch {
        match client.get_file(path, branch).await {
            Ok(file) => {
                let text = file.content.unwrap_or_default();
                return Ok(LoadedFile::Existing {
                    text,
                    sha: file.sha,
                });
            }
            Err(e) if e.contains("not found") => {}
            Err(e) => return Err(e),
        }
    }

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

async fn create_editor_branch(client: &GitHubClient, path: &str) -> Result<String, String> {
    let source_sha = client.get_branch_sha(DEFAULT_BRANCH).await?;

    let today = js_sys::Date::new_0();
    let year = today.get_full_year();
    let month = today.get_month() + 1;
    let day = today.get_date();
    let date_str = format!("{year}-{month:02}-{day:02}");

    let slug = crate::utils::slug_from_path(path);
    let branch_name = format!("editor/{date_str}-{slug}");

    client.create_branch(&branch_name, &source_sha).await?;
    Ok(branch_name)
}

// ── Template generation ─────────────────────────────────────────

fn generate_template(path: &str) -> String {
    let today = js_sys::Date::new_0();
    let year = today.get_full_year();
    let month = today.get_month() + 1;
    let day = today.get_date();
    let date_str = format!("{year}-{month:02}-{day:02}");

    let slug = crate::utils::slug_from_path(path);
    let title = crate::utils::title_from_slug(&slug);

    format!(
        r#"+++
title = "{title}"
date = "{date_str}"
description = ""
draft = true
# taxonomies.tags = ["comics", "games", "backlog", "movies", "tv", "whats-good"]
+++
"#
    )
}

// ── Image upload helpers ─────────────────────────────────────────

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

/// Wrap selected text (or insert placeholder) with prefix/suffix markers.
/// Returns the new full content string.
fn apply_format_to_content(
    textarea: &HtmlTextAreaElement,
    current: &str,
    prefix: &str,
    suffix: &str,
    placeholder: &str,
) -> String {
    let start = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
    let end = textarea
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or(start as u32) as usize;
    let byte_start = crate::utils::char_pos_to_byte_offset(current, start);
    let byte_end = crate::utils::char_pos_to_byte_offset(current, end);
    let selected = &current[byte_start..byte_end];
    let inner = if selected.is_empty() {
        placeholder
    } else {
        selected
    };
    format!(
        "{}{}{}{}{}",
        &current[..byte_start],
        prefix,
        inner,
        suffix,
        &current[byte_end..]
    )
}

// ── Debounce / highlighting helpers ─────────────────────────────

async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let _ = gloo_utils::window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn highlight_code_blocks() {
    let _ = js_sys::eval(
        "if(typeof hljs!=='undefined'){document.querySelectorAll('pre code:not(.hljs)').forEach(el=>hljs.highlightElement(el));}",
    );
}

// ── History panel ────────────────────────────────────────────────

fn render_history_panel(
    commits: &[CommitSummary],
    loading: bool,
    error: &Option<String>,
    on_select: Callback<String>,
) -> Html {
    html! {
        <div class="history-panel">
            <div class="history-panel-header">
                <span class="history-panel-title">{"Post history"}</span>
            </div>
            if loading {
                <p class="history-loading">{"Loading history\u{2026}"}</p>
            } else if let Some(ref err) = error {
                <p class="error">{err}</p>
            } else if commits.is_empty() {
                <p class="history-empty">{"No commits found."}</p>
            } else {
                <div class="history-list">
                    { for commits.iter().map(|c| {
                        let sha = c.sha.clone();
                        let on_select = on_select.clone();
                        let onclick = Callback::from(move |_: MouseEvent| {
                            on_select.emit(sha.clone());
                        });
                        let short_sha = &c.sha[..7.min(c.sha.len())];
                        let short_msg = if c.message.len() > 60 {
                            format!("{}\u{2026}", &c.message[..60])
                        } else {
                            c.message.clone()
                        };
                        html! {
                            <div class="history-item" onclick={onclick}>
                                <div class="history-item-top">
                                    <span class="history-sha">{short_sha}</span>
                                    <span class="history-date">{format_history_date(&c.date)}</span>
                                </div>
                                <div class="history-item-bottom">
                                    <span class="history-msg">{short_msg}</span>
                                    <span class="history-stats">
                                        <span class="history-add">{format!("+{}", c.additions)}</span>
                                        {" "}
                                        <span class="history-del">{format!("-{}", c.deletions)}</span>
                                    </span>
                                </div>
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}

/// Format an ISO 8601 date string for display in the history panel.
fn format_history_date(iso: &str) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(iso));
    if d.get_time().is_nan() {
        return iso.to_string();
    }
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        d.get_utc_full_year(),
        d.get_utc_month() + 1,
        d.get_utc_date(),
        d.get_utc_hours(),
        d.get_utc_minutes()
    )
}
