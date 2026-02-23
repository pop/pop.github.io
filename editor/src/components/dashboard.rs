use std::cmp::Ordering;
use std::collections::HashMap;

use gloo_storage::{SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use futures::future::join_all;

use crate::app::AuthContext;
use crate::models::github::{CompareResponse, ContentEntry, DiffFile, GitRef};
use crate::models::post::{extract_frontmatter, parse_frontmatter};
use crate::routes::Route;
use crate::services::github::GitHubClient;

const DEFAULT_BRANCH: &str = "source";
const SORT_KEY: &str = "dashboard_sort_mode";
const RETURN_PATH_KEY: &str = "dashboard_return_path";
const ALL_FILES_KEY: &str = "all_files_index";
const CACHE_TTL_MS: f64 = 5.0 * 60.0 * 1000.0; // 5 minutes
const ALL_FILES_TTL_MS: f64 = 15.0 * 60.0 * 1000.0; // 15 minutes

#[derive(Clone, PartialEq)]
enum PostStatus {
    Draft,
    Published,
    NoFrontmatter,
}

fn detect_post_status(content: &str) -> PostStatus {
    if extract_frontmatter(content).is_none() {
        return PostStatus::NoFrontmatter;
    }
    let fields = parse_frontmatter(content);
    match fields
        .iter()
        .find(|(k, _)| k == "draft")
        .map(|(_, v)| v.as_str())
    {
        Some("true") => PostStatus::Draft,
        _ => PostStatus::Published,
    }
}

#[derive(Clone, PartialEq)]
enum CiState {
    Pending,
    Success,
    Failure(String),
    None,
}

#[derive(Clone, Copy, PartialEq)]
enum SortMode {
    Alphabetical,
    LastModified,
}

#[derive(Clone, Copy, PartialEq)]
enum StatusFilter {
    All,
    Draft,
    Published,
}

// ── Directory listing cache ─────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct CachedListing {
    entries: Vec<ContentEntry>,
    timestamp: f64,
    #[serde(default)]
    commit_dates: Option<HashMap<String, f64>>,
    #[serde(default)]
    post_statuses: Option<HashMap<String, String>>,
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
        commit_dates: None,
        post_statuses: None,
    };
    let _ = SessionStorage::set(&cache_key(path), &cached);
}

fn get_cached_commit_dates(path: &str) -> Option<HashMap<String, f64>> {
    let cached: CachedListing = SessionStorage::get(&cache_key(path)).ok()?;
    cached.commit_dates
}

fn set_cached_commit_dates(path: &str, dates: &HashMap<String, f64>) {
    if let Ok(mut cached) = SessionStorage::get::<CachedListing>(&cache_key(path)) {
        cached.commit_dates = Some(dates.clone());
        let _ = SessionStorage::set(&cache_key(path), &cached);
    }
}

fn get_cached_post_statuses(path: &str) -> Option<HashMap<String, String>> {
    let cached: CachedListing = SessionStorage::get(&cache_key(path)).ok()?;
    cached.post_statuses
}

fn set_cached_post_statuses(path: &str, statuses: &HashMap<String, String>) {
    if let Ok(mut cached) = SessionStorage::get::<CachedListing>(&cache_key(path)) {
        cached.post_statuses = Some(statuses.clone());
        let _ = SessionStorage::set(&cache_key(path), &cached);
    }
}

// ── Global file index cache ─────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct CachedAllFiles {
    entries: Vec<ContentEntry>,
    timestamp: f64,
}

fn all_files_cache_key(branch: Option<&str>) -> String {
    match branch {
        Some(b) => format!("{ALL_FILES_KEY}_{b}"),
        None => ALL_FILES_KEY.to_string(),
    }
}

fn get_cached_all_files(branch: Option<&str>) -> Option<Vec<ContentEntry>> {
    let key = all_files_cache_key(branch);
    let cached: CachedAllFiles = SessionStorage::get(&key).ok()?;
    let now = js_sys::Date::now();
    if now - cached.timestamp < ALL_FILES_TTL_MS {
        Some(cached.entries)
    } else {
        SessionStorage::delete(&key);
        None
    }
}

fn set_cached_all_files(entries: &[ContentEntry], branch: Option<&str>) {
    let cached = CachedAllFiles {
        entries: entries.to_vec(),
        timestamp: js_sys::Date::now(),
    };
    let _ = SessionStorage::set(&all_files_cache_key(branch), &cached);
}

/// Invalidate a single directory cache entry.
pub fn invalidate_cache(path: &str) {
    SessionStorage::delete(&cache_key(path));
}

/// Invalidate all directory cache entries.
pub fn invalidate_all_caches() {
    let storage = gloo_utils::window().session_storage().ok().flatten();
    let Some(storage) = storage else { return };
    let len = storage.length().unwrap_or(0);
    let mut keys_to_delete = Vec::new();
    for i in 0..len {
        if let Ok(Some(key)) = storage.key(i) {
            if key.starts_with("dir_cache_") || key.starts_with("all_files_index") {
                keys_to_delete.push(key);
            }
        }
    }
    for key in keys_to_delete {
        let _ = storage.delete(&key);
    }
}

fn is_media_file(name: &str) -> bool {
    matches!(
        name.rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase()
            .as_str(),
        "jpg"
            | "jpeg"
            | "png"
            | "gif"
            | "webp"
            | "svg"
            | "mp4"
            | "webm"
            | "mov"
            | "avi"
            | "mp3"
            | "wav"
            | "ogg"
            | "flac"
            | "pdf"
            | "zip"
            | "gz"
            | "tar"
    )
}

// ── Dashboard component ─────────────────────────────────────────

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");

    let current_path = use_state(|| {
        SessionStorage::get::<String>(RETURN_PATH_KEY)
            .ok()
            .map(|path| {
                SessionStorage::delete(RETURN_PATH_KEY);
                path
            })
            .unwrap_or_else(|| "content".to_string())
    });
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

    // Phase 8a: search/filter
    let filter_text = use_state(String::new);

    // Phase 8b: sort options
    let sort_mode = use_state(|| {
        SessionStorage::get::<String>(SORT_KEY)
            .ok()
            .filter(|s| s == "recent")
            .map(|_| SortMode::LastModified)
            .unwrap_or(SortMode::Alphabetical)
    });
    let commit_dates = use_state(HashMap::<String, f64>::new);
    let dates_loading = use_state(|| false);

    // Phase 15: draft/published status icons
    let post_statuses = use_state(HashMap::<String, PostStatus>::new);

    // Folder single-.md status: maps folder_path → PostStatus of its single .md child
    let folder_md_statuses = use_state(HashMap::<String, PostStatus>::new);

    // Phase 17: post status filter
    let status_filter = use_state(|| StatusFilter::All);

    // Global file index for search (Bug 3)
    let all_files = use_state(|| Vec::<ContentEntry>::new());
    let all_files_loading = use_state(|| false);

    // Delete confirmation state
    let delete_confirm = use_state(|| Option::<ContentEntry>::None);
    let delete_error = use_state(|| Option::<String>::None);
    let deleting = use_state(|| false);

    // Branch publish/discard/CI state
    let show_diff = use_state(|| false);
    let diff_data = use_state(|| Option::<CompareResponse>::None);
    let diff_loading = use_state(|| false);
    let ci_status = use_state(|| CiState::None);
    let branch_saving = use_state(|| false);
    let branch_error = use_state(|| Option::<String>::None);
    let show_ci_warning_modal = use_state(|| false);
    let syncing = use_state(|| false);

    // Active branch from shared context (replaces local use_state)
    let active_branch_opt = auth.active_branch.clone();
    let set_active_branch = auth.set_active_branch.clone();

    let is_authenticated = auth.token.is_some();

    // Fetch editor branches on mount (only when authenticated)
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
        let branch = active_branch_opt.clone();
        let commit_dates = commit_dates.clone();

        use_effect_with(
            (path.clone(), token.clone(), refresh, branch.clone()),
            move |_| {
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
                    // Restore cached commit dates if available
                    if let Some(dates) = get_cached_commit_dates(&path) {
                        commit_dates.set(dates);
                    } else {
                        commit_dates.set(HashMap::new());
                    }
                } else {
                    loading.set(true);
                    error.set(None);
                    commit_dates.set(HashMap::new());

                    let client = match token {
                        Some(t) => GitHubClient::new(t),
                        None => GitHubClient::anonymous(),
                    };

                    wasm_bindgen_futures::spawn_local(async move {
                        match client.list_contents(&path, branch.as_deref()).await {
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

                || ()
            },
        );
    }

    // Load the global file index; re-fetch whenever the active branch changes
    {
        let all_files = all_files.clone();
        let all_files_loading = all_files_loading.clone();
        let token = auth.token.clone();
        let active_branch_opt = active_branch_opt.clone();
        use_effect_with(active_branch_opt.clone(), move |_| {
            all_files.set(vec![]);
            let branch = active_branch_opt.clone();
            if let Some(cached) = get_cached_all_files(branch.as_deref()) {
                all_files.set(cached);
            } else {
                all_files_loading.set(true);
                let client = match token {
                    Some(t) => GitHubClient::new(t),
                    None => GitHubClient::anonymous(),
                };
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(files) = client.list_all_files(branch.as_deref()).await {
                        set_cached_all_files(&files, branch.as_deref());
                        all_files.set(files);
                    }
                    all_files_loading.set(false);
                });
            }
            || ()
        });
    }

    // Auto-fetch commit dates when sort is restored to LastModified on mount (Bug 1)
    {
        let sort_mode = sort_mode.clone();
        let commit_dates = commit_dates.clone();
        let dates_loading = dates_loading.clone();
        let entries = entries.clone();
        let token = auth.token.clone();
        let current_path = current_path.clone();
        let active_branch_opt = active_branch_opt.clone();
        use_effect_with((*sort_mode, entries.len()), move |_| {
            if *sort_mode == SortMode::LastModified
                && commit_dates.is_empty()
                && !entries.is_empty()
            {
                let dir_path = (*current_path).clone();
                if let Some(cached) = get_cached_commit_dates(&dir_path) {
                    commit_dates.set(cached);
                } else {
                    let commit_dates = commit_dates.clone();
                    let dates_loading = dates_loading.clone();
                    let paths: Vec<String> = entries.iter().map(|e| e.path.clone()).collect();
                    let branch = active_branch_opt.clone();
                    let client = match token {
                        Some(t) => GitHubClient::new(t),
                        None => GitHubClient::anonymous(),
                    };
                    dates_loading.set(true);
                    wasm_bindgen_futures::spawn_local(async move {
                        let dates = client
                            .get_commit_dates_bulk(&paths, branch.as_deref())
                            .await;
                        set_cached_commit_dates(&dir_path, &dates);
                        commit_dates.set(dates);
                        dates_loading.set(false);
                    });
                }
            }
            || ()
        });
    }

    // Phase 15: fetch draft/published status for .md files in the current directory
    {
        let post_statuses = post_statuses.clone();
        let token = auth.token.clone();
        let current_path = current_path.clone();
        let entries = entries.clone();
        let active_branch_opt = active_branch_opt.clone();

        use_effect_with(((*current_path).clone(), entries.len()), move |_| {
            post_statuses.set(HashMap::new());

            let md_paths: Vec<String> = entries
                .iter()
                .filter(|e| e.entry_type == "file" && e.name.ends_with(".md"))
                .map(|e| e.path.clone())
                .collect();

            if !md_paths.is_empty() {
                let path = (*current_path).clone();

                // Check cache first
                if let Some(cached) = get_cached_post_statuses(&path) {
                    let parsed: HashMap<String, PostStatus> = cached
                        .into_iter()
                        .map(|(k, v)| {
                            let status = match v.as_str() {
                                "draft" => PostStatus::Draft,
                                "published" => PostStatus::Published,
                                _ => PostStatus::NoFrontmatter,
                            };
                            (k, status)
                        })
                        .collect();
                    post_statuses.set(parsed);
                } else {
                    // Fetch file contents in parallel
                    let branch = active_branch_opt
                        .clone()
                        .unwrap_or_else(|| DEFAULT_BRANCH.to_string());

                    wasm_bindgen_futures::spawn_local(async move {
                        let futures: Vec<_> = md_paths
                            .into_iter()
                            .map(|file_path| {
                                let token = token.clone();
                                let branch = branch.clone();
                                async move {
                                    let client = match token {
                                        Some(t) => GitHubClient::new(t),
                                        None => GitHubClient::anonymous(),
                                    };
                                    let result = client.get_file(&file_path, &branch).await;
                                    (file_path, result)
                                }
                            })
                            .collect();

                        let results = join_all(futures).await;

                        let mut statuses_map: HashMap<String, PostStatus> = HashMap::new();
                        let mut cache_map: HashMap<String, String> = HashMap::new();

                        for (file_path, result) in results {
                            let status = match result {
                                Ok(fc) => detect_post_status(fc.content.as_deref().unwrap_or("")),
                                Err(_) => PostStatus::NoFrontmatter,
                            };
                            let status_str = match &status {
                                PostStatus::Draft => "draft",
                                PostStatus::Published => "published",
                                PostStatus::NoFrontmatter => "none",
                            };
                            cache_map.insert(file_path.clone(), status_str.to_string());
                            statuses_map.insert(file_path, status);
                        }

                        set_cached_post_statuses(&path, &cache_map);
                        post_statuses.set(statuses_map);
                    });
                }
            }

            || ()
        });
    }

    // Folder single-.md status effect: for each dir entry, if it has exactly one .md child,
    // fetch that child and detect draft/published status.
    {
        let folder_md_statuses = folder_md_statuses.clone();
        let token = auth.token.clone();
        let current_path = current_path.clone();
        let entries = entries.clone();
        let active_branch_opt = active_branch_opt.clone();

        use_effect_with(((*current_path).clone(), entries.len()), move |_| {
            folder_md_statuses.set(HashMap::new());

            let dir_entries: Vec<ContentEntry> = entries
                .iter()
                .filter(|e| e.entry_type == "dir")
                .cloned()
                .collect();

            if !dir_entries.is_empty() {
                let branch = active_branch_opt
                    .clone()
                    .unwrap_or_else(|| DEFAULT_BRANCH.to_string());

                wasm_bindgen_futures::spawn_local(async move {
                    // Phase 1: fetch listing for each dir (use cache if available)
                    let listing_futures: Vec<_> = dir_entries
                        .iter()
                        .map(|dir| {
                            let dir_path = dir.path.clone();
                            let token = token.clone();
                            let branch = branch.clone();
                            async move {
                                let children = if let Some(cached) = get_cached_listing(&dir_path) {
                                    Some(cached)
                                } else {
                                    let client = match token {
                                        Some(ref t) => GitHubClient::new(t.clone()),
                                        None => GitHubClient::anonymous(),
                                    };
                                    client.list_contents(&dir_path, Some(&branch)).await.ok()
                                };
                                (dir_path, children)
                            }
                        })
                        .collect();

                    let listings = join_all(listing_futures).await;

                    // Phase 2: collect dirs with exactly one .md child, fetch those files
                    let single_md_futures: Vec<_> = listings
                        .into_iter()
                        .filter_map(|(dir_path, children)| {
                            let children = children?;
                            // [a066f2] Directories that contain sub-directories are sections,
                            // not posts — always show the folder icon for these.
                            if children.iter().any(|e| e.entry_type == "dir") {
                                return None;
                            }
                            let md_children: Vec<ContentEntry> = children
                                .into_iter()
                                .filter(|e| e.entry_type == "file" && e.name.ends_with(".md"))
                                .collect();
                            if md_children.len() == 1 {
                                Some((dir_path, md_children.into_iter().next().unwrap()))
                            } else {
                                None
                            }
                        })
                        .map(|(dir_path, md_entry)| {
                            let token = token.clone();
                            let branch = branch.clone();
                            async move {
                                let client = match token {
                                    Some(ref t) => GitHubClient::new(t.clone()),
                                    None => GitHubClient::anonymous(),
                                };
                                let result = client.get_file(&md_entry.path, &branch).await;
                                (dir_path, result)
                            }
                        })
                        .collect();

                    let file_results = join_all(single_md_futures).await;

                    let mut map: HashMap<String, PostStatus> = HashMap::new();
                    for (dir_path, result) in file_results {
                        // [469284] For folder icons, treat any file with frontmatter that
                        // isn't explicitly draft=true as Published. This differs from
                        // file-level rendering where absent draft key shows no icon.
                        let status = match result {
                            Ok(fc) => {
                                let content = fc.content.as_deref().unwrap_or("");
                                if extract_frontmatter(content).is_none() {
                                    PostStatus::NoFrontmatter
                                } else {
                                    let fields = parse_frontmatter(content);
                                    if fields.iter().any(|(k, v)| k == "draft" && v == "true") {
                                        PostStatus::Draft
                                    } else {
                                        PostStatus::Published
                                    }
                                }
                            }
                            Err(_) => PostStatus::NoFrontmatter,
                        };
                        if matches!(status, PostStatus::Draft | PostStatus::Published) {
                            map.insert(dir_path, status);
                        }
                    }

                    folder_md_statuses.set(map);
                });
            }

            || ()
        });
    }

    // CI status: fetch on mount and poll every 15s while pending
    {
        let ci_status = ci_status.clone();
        let branch_val = active_branch_opt.clone();
        let token = auth.token.clone();

        use_effect_with((branch_val.clone(), token.clone()), move |_| {
            let ci_status = ci_status.clone();
            let mut interval_handle: Option<i32> = None;

            if let (Some(branch_name), Some(token)) = (branch_val, token) {
                {
                    let ci_status = ci_status.clone();
                    let branch_name = branch_name.clone();
                    let token = token.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        fetch_ci_status(&token, &branch_name, &ci_status).await;
                    });
                }

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

    let on_refresh = {
        let force_refresh = force_refresh.clone();
        Callback::from(move |_: MouseEvent| {
            invalidate_all_caches();
            force_refresh.set(*force_refresh + 1);
        })
    };

    let on_publish = {
        let branch_opt = active_branch_opt.clone();
        let token = auth.token.clone();
        let show_diff = show_diff.clone();
        let diff_data = diff_data.clone();
        let diff_loading = diff_loading.clone();
        let branch_error = branch_error.clone();
        let ci_status_for_publish = ci_status.clone();
        let show_ci_warning_modal = show_ci_warning_modal.clone();

        Callback::from(move |_: MouseEvent| {
            let Some(branch_name) = branch_opt.clone() else {
                branch_error.set(Some("No active branch to publish".into()));
                return;
            };
            let Some(token) = token.clone() else { return };

            // Show warning modal if CI has not yet passed
            if matches!(&*ci_status_for_publish, CiState::Pending | CiState::None) {
                show_ci_warning_modal.set(true);
                return;
            }

            let show_diff = show_diff.clone();
            let diff_data = diff_data.clone();
            let diff_loading = diff_loading.clone();
            let branch_error = branch_error.clone();

            diff_loading.set(true);
            show_diff.set(false);
            branch_error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let client = GitHubClient::new(token);
                match client.compare_branches(DEFAULT_BRANCH, &branch_name).await {
                    Ok(data) => {
                        diff_data.set(Some(data));
                        show_diff.set(true);
                        diff_loading.set(false);
                    }
                    Err(e) => {
                        branch_error.set(Some(e));
                        diff_loading.set(false);
                    }
                }
            });
        })
    };

    let on_ci_warning_publish = {
        let branch_opt = active_branch_opt.clone();
        let token = auth.token.clone();
        let show_diff = show_diff.clone();
        let diff_data = diff_data.clone();
        let diff_loading = diff_loading.clone();
        let branch_error = branch_error.clone();
        let show_ci_warning_modal = show_ci_warning_modal.clone();

        Callback::from(move |_: MouseEvent| {
            let Some(branch_name) = branch_opt.clone() else {
                return;
            };
            let Some(token) = token.clone() else { return };

            show_ci_warning_modal.set(false);

            let show_diff = show_diff.clone();
            let diff_data = diff_data.clone();
            let diff_loading = diff_loading.clone();
            let branch_error = branch_error.clone();

            diff_loading.set(true);
            show_diff.set(false);
            branch_error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let client = GitHubClient::new(token);
                match client.compare_branches(DEFAULT_BRANCH, &branch_name).await {
                    Ok(data) => {
                        diff_data.set(Some(data));
                        show_diff.set(true);
                        diff_loading.set(false);
                    }
                    Err(e) => {
                        branch_error.set(Some(e));
                        diff_loading.set(false);
                    }
                }
            });
        })
    };

    let on_ci_warning_cancel = {
        let show_ci_warning_modal = show_ci_warning_modal.clone();
        Callback::from(move |_: MouseEvent| {
            show_ci_warning_modal.set(false);
        })
    };

    let on_confirm_publish = {
        let branch_opt = active_branch_opt.clone();
        let set_active_branch = set_active_branch.clone();
        let token = auth.token.clone();
        let show_diff = show_diff.clone();
        let branch_saving = branch_saving.clone();
        let branch_error = branch_error.clone();
        let show_branches = show_branches.clone();
        let branches = branches.clone();

        Callback::from(move |_: MouseEvent| {
            let Some(branch_name) = branch_opt.clone() else {
                branch_error.set(Some("No active branch to publish".into()));
                return;
            };
            let Some(token) = token.clone() else { return };

            let set_active_branch = set_active_branch.clone();
            let show_diff = show_diff.clone();
            let branch_saving = branch_saving.clone();
            let branch_error = branch_error.clone();
            let show_branches = show_branches.clone();
            let branches = branches.clone();

            show_diff.set(false);
            show_branches.set(false);
            branch_saving.set(true);
            branch_error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let client = GitHubClient::new(token);
                let message = format!("Publish {branch_name}");

                match client
                    .merge_branch(&branch_name, DEFAULT_BRANCH, &message)
                    .await
                {
                    Ok(()) => {
                        let _ = client.delete_branch(&branch_name).await;
                        let full_ref = format!("refs/heads/{branch_name}");
                        branches.set(
                            (*branches)
                                .iter()
                                .filter(|b| b.ref_name != full_ref)
                                .cloned()
                                .collect(),
                        );
                        set_active_branch.emit(None);
                        invalidate_all_caches();
                        branch_saving.set(false);
                    }
                    Err(e) => {
                        branch_error.set(Some(e));
                        branch_saving.set(false);
                    }
                }
            });
        })
    };

    let on_cancel_diff = {
        let show_diff = show_diff.clone();
        Callback::from(move |_: MouseEvent| {
            show_diff.set(false);
        })
    };

    let on_discard = {
        let branch_opt = active_branch_opt.clone();
        let set_active_branch = set_active_branch.clone();
        let token = auth.token.clone();
        let branch_saving = branch_saving.clone();
        let branch_error = branch_error.clone();
        let show_branches = show_branches.clone();
        let branches = branches.clone();

        Callback::from(move |_: MouseEvent| {
            let window = gloo_utils::window();
            if !window
                .confirm_with_message("Discard all changes? This will delete the editor branch.")
                .unwrap_or(false)
            {
                return;
            }

            let Some(branch_name) = branch_opt.clone() else {
                branch_error.set(Some("No active branch to discard".into()));
                return;
            };
            let Some(token) = token.clone() else { return };

            let set_active_branch = set_active_branch.clone();
            let branch_saving = branch_saving.clone();
            let branch_error = branch_error.clone();
            let show_branches = show_branches.clone();
            let branches = branches.clone();

            show_branches.set(false);
            branch_saving.set(true);
            branch_error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let client = GitHubClient::new(token);
                match client.delete_branch(&branch_name).await {
                    Ok(()) => {
                        let full_ref = format!("refs/heads/{branch_name}");
                        branches.set(
                            (*branches)
                                .iter()
                                .filter(|b| b.ref_name != full_ref)
                                .cloned()
                                .collect(),
                        );
                        set_active_branch.emit(None);
                        invalidate_all_caches();
                        branch_saving.set(false);
                    }
                    Err(e) => {
                        branch_error.set(Some(e));
                        branch_saving.set(false);
                    }
                }
            });
        })
    };

    let on_sync = {
        let branch_opt = active_branch_opt.clone();
        let syncing = syncing.clone();
        let branch_error = branch_error.clone();
        let token = auth.token.clone();

        Callback::from(move |_: MouseEvent| {
            let Some(branch_name) = branch_opt.clone() else {
                return;
            };
            let Some(token) = token.clone() else { return };

            let syncing = syncing.clone();
            let branch_error = branch_error.clone();

            syncing.set(true);
            branch_error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let client = GitHubClient::new(token);
                let message = format!("Sync {DEFAULT_BRANCH} into {branch_name}");
                match client
                    .merge_branch(DEFAULT_BRANCH, &branch_name, &message)
                    .await
                {
                    Ok(()) => {
                        syncing.set(false);
                    }
                    Err(e) => {
                        branch_error.set(Some(e));
                        syncing.set(false);
                    }
                }
            });
        })
    };

    let on_navigate = {
        let current_path = current_path.clone();
        let navigator = navigator.clone();
        let filter_text = filter_text.clone();
        let status_filter = status_filter.clone();
        Callback::from(move |entry: ContentEntry| {
            if entry.entry_type == "dir" {
                filter_text.set(String::new());
                status_filter.set(StatusFilter::All);
                current_path.set(entry.path);
            } else {
                let _ = SessionStorage::set(RETURN_PATH_KEY, (*current_path).clone());
                navigator.push(&Route::Editor { path: entry.path });
            }
        })
    };

    // Navigate directly to a file path (used from global search results)
    let on_navigate_to_file = {
        let navigator = navigator.clone();
        let current_path = current_path.clone();
        Callback::from(move |path: String| {
            let _ = SessionStorage::set(RETURN_PATH_KEY, (*current_path).clone());
            navigator.push(&Route::Editor { path });
        })
    };

    let on_navigate_up = {
        let current_path = current_path.clone();
        let filter_text = filter_text.clone();
        let status_filter = status_filter.clone();
        Callback::from(move |_: MouseEvent| {
            let path = (*current_path).clone();
            if let Some(pos) = path.rfind('/') {
                filter_text.set(String::new());
                status_filter.set(StatusFilter::All);
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
        let set_active_branch = set_active_branch.clone();
        let force_refresh = force_refresh.clone();
        Callback::from(move |name: String| {
            set_active_branch.emit(Some(name));
            invalidate_all_caches();
            force_refresh.set(*force_refresh + 1);
        })
    };

    let on_clear_branch = {
        let set_active_branch = set_active_branch.clone();
        let force_refresh = force_refresh.clone();
        Callback::from(move |_: MouseEvent| {
            set_active_branch.emit(None);
            invalidate_all_caches();
            force_refresh.set(*force_refresh + 1);
        })
    };

    // Phase 8a: filter callbacks
    let on_filter_input = {
        let filter_text = filter_text.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            filter_text.set(target.value());
        })
    };

    let on_clear_filter = {
        let filter_text = filter_text.clone();
        Callback::from(move |_: MouseEvent| {
            filter_text.set(String::new());
        })
    };

    // Phase 8b: sort toggle with lazy date fetching
    let on_set_sort_alpha = {
        let sort_mode = sort_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let _ = SessionStorage::set(SORT_KEY, "alpha");
            sort_mode.set(SortMode::Alphabetical);
        })
    };

    let on_set_sort_recent = {
        let sort_mode = sort_mode.clone();
        let commit_dates = commit_dates.clone();
        let dates_loading = dates_loading.clone();
        let entries = entries.clone();
        let token = auth.token.clone();
        let current_path = current_path.clone();
        let active_branch_opt = active_branch_opt.clone();
        Callback::from(move |_: MouseEvent| {
            let _ = SessionStorage::set(SORT_KEY, "recent");
            sort_mode.set(SortMode::LastModified);

            // If we already have dates, nothing to do
            if !commit_dates.is_empty() {
                return;
            }

            // Check cache
            if let Some(cached) = get_cached_commit_dates(&current_path) {
                commit_dates.set(cached);
                return;
            }

            {
                let commit_dates = commit_dates.clone();
                let dates_loading = dates_loading.clone();
                let paths: Vec<String> = entries.iter().map(|e| e.path.clone()).collect();
                let branch = active_branch_opt.clone();
                let dir_path = (*current_path).clone();

                let client = match token.clone() {
                    Some(t) => GitHubClient::new(t),
                    None => GitHubClient::anonymous(),
                };

                dates_loading.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    let dates = client
                        .get_commit_dates_bulk(&paths, branch.as_deref())
                        .await;
                    set_cached_commit_dates(&dir_path, &dates);
                    commit_dates.set(dates);
                    dates_loading.set(false);
                });
            }
        })
    };

    let on_delete_request = {
        let delete_confirm = delete_confirm.clone();
        let delete_error = delete_error.clone();
        Callback::from(move |entry: ContentEntry| {
            delete_error.set(None);
            delete_confirm.set(Some(entry));
        })
    };

    // Phase 17: status filter callbacks
    let on_filter_all = {
        let status_filter = status_filter.clone();
        Callback::from(move |_: MouseEvent| status_filter.set(StatusFilter::All))
    };
    let on_filter_draft = {
        let status_filter = status_filter.clone();
        Callback::from(move |_: MouseEvent| status_filter.set(StatusFilter::Draft))
    };
    let on_filter_published = {
        let status_filter = status_filter.clone();
        Callback::from(move |_: MouseEvent| status_filter.set(StatusFilter::Published))
    };

    let on_delete_cancel = {
        let delete_confirm = delete_confirm.clone();
        let delete_error = delete_error.clone();
        Callback::from(move |_: MouseEvent| {
            delete_confirm.set(None);
            delete_error.set(None);
        })
    };

    let on_delete_confirm = {
        let delete_confirm = delete_confirm.clone();
        let delete_error = delete_error.clone();
        let deleting = deleting.clone();
        let all_files = all_files.clone();
        let force_refresh = force_refresh.clone();
        let active_branch_opt = active_branch_opt.clone();
        let token = auth.token.clone();
        Callback::from(move |_: MouseEvent| {
            let Some(entry) = (*delete_confirm).clone() else {
                return;
            };
            let active_branch_for_cache = active_branch_opt.clone();
            let branch = active_branch_opt
                .clone()
                .unwrap_or_else(|| "source".to_string());
            let Some(token) = token.clone() else { return };
            let current_refresh = *force_refresh;
            deleting.set(true);
            let delete_confirm = delete_confirm.clone();
            let delete_error = delete_error.clone();
            let deleting = deleting.clone();
            let all_files = all_files.clone();
            let force_refresh = force_refresh.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = GitHubClient::new(token);
                let message = format!("Delete {}", entry.name);
                match client
                    .delete_file(&entry.path, &entry.sha, &message, &branch)
                    .await
                {
                    Ok(()) => {
                        let path = entry.path.clone();
                        all_files.set(
                            (*all_files)
                                .iter()
                                .filter(|f| f.path != path)
                                .cloned()
                                .collect(),
                        );
                        invalidate_all_caches();
                        SessionStorage::delete(&all_files_cache_key(
                            active_branch_for_cache.as_deref(),
                        ));
                        force_refresh.set(current_refresh + 1);
                        delete_confirm.set(None);
                    }
                    Err(e) => {
                        delete_error.set(Some(e));
                    }
                }
                deleting.set(false);
            });
        })
    };

    let breadcrumbs = render_breadcrumbs(&current_path, {
        let current_path = current_path.clone();
        let filter_text = filter_text.clone();
        Callback::from(move |path: String| {
            filter_text.set(String::new());
            current_path.set(path);
        })
    });

    // Compute filtered and sorted entries for display
    let display_entries: Vec<ContentEntry> = {
        let filter = (*filter_text).to_lowercase();
        let cur_status_filter = *status_filter;
        let mut filtered: Vec<ContentEntry> = (*entries)
            .iter()
            .filter(|e| {
                // Text filter
                if !filter.is_empty() && !e.name.to_lowercase().contains(&filter) {
                    return false;
                }
                // Status filter: applies to .md files and known folder-posts
                if cur_status_filter != StatusFilter::All {
                    if e.name.ends_with(".md") {
                        let status = post_statuses.get(&e.path);
                        match cur_status_filter {
                            StatusFilter::Draft => {
                                if !matches!(status, Some(PostStatus::Draft)) {
                                    return false;
                                }
                            }
                            StatusFilter::Published => {
                                if !matches!(status, Some(PostStatus::Published)) {
                                    return false;
                                }
                            }
                            StatusFilter::All => {}
                        }
                    } else if e.entry_type == "dir" {
                        if let Some(status) = folder_md_statuses.get(&e.path) {
                            match cur_status_filter {
                                StatusFilter::Draft => {
                                    if !matches!(status, PostStatus::Draft) {
                                        return false;
                                    }
                                }
                                StatusFilter::Published => {
                                    if !matches!(status, PostStatus::Published) {
                                        return false;
                                    }
                                }
                                StatusFilter::All => {}
                            }
                        }
                    }
                }
                true
            })
            .cloned()
            .collect();

        if *sort_mode == SortMode::LastModified {
            let dates = &*commit_dates;
            filtered.sort_by(|a, b| {
                let type_ord = match (a.entry_type.as_str(), b.entry_type.as_str()) {
                    ("dir", "file") => Ordering::Less,
                    ("file", "dir") => Ordering::Greater,
                    _ => Ordering::Equal,
                };
                type_ord.then_with(|| {
                    let a_date = dates.get(&a.path).copied().unwrap_or(0.0);
                    let b_date = dates.get(&b.path).copied().unwrap_or(0.0);
                    b_date.partial_cmp(&a_date).unwrap_or(Ordering::Equal)
                })
            });
        }

        filtered
    };

    let cur_sort = *sort_mode;
    let cur_dates = (*commit_dates).clone();
    let cur_post_statuses = (*post_statuses).clone();
    let cur_folder_md_statuses = (*folder_md_statuses).clone();

    // Global search results (used when filter_text is non-empty)
    let filter_active = !filter_text.is_empty();
    let global_search_results: Vec<ContentEntry> = if filter_active {
        let filter_lower = (*filter_text).to_lowercase();
        (*all_files)
            .iter()
            .filter(|e| e.path.to_lowercase().contains(&filter_lower))
            .cloned()
            .collect()
    } else {
        vec![]
    };

    let ci_blocks_publish = matches!(*ci_status, CiState::Failure(_));
    let publish_btn_class = match &*ci_status {
        CiState::Pending | CiState::None => "publish-btn pending",
        CiState::Failure(_) => "publish-btn failure",
        CiState::Success => "publish-btn",
    };

    html! {
        <div class="dashboard">
            <div class="dashboard-header">
                <div class="dashboard-title-row">
                    <h2>{"editor.elijah.run"}</h2>
                    <div class="dashboard-actions">
                        <button class="refresh-btn" onclick={on_refresh} title="Refresh">
                            {"\u{21BB}"}
                        </button>
                        if is_authenticated {
                            <button class="branch-toggle-btn" onclick={toggle_branches}>
                                { if *show_branches { "Hide branches" } else { "Branches" } }
                            </button>
                            <button class="new-post-btn" onclick={toggle_new_post}>
                                { if *show_new_post { "Cancel" } else { "+ New Post" } }
                            </button>
                        }
                    </div>
                </div>
                if *show_branches {
                    {render_branch_list(&branches, &active_branch_opt, on_select_branch.clone(), on_clear_branch.clone())}
                }
                if let Some(ref name) = active_branch_opt {
                    <div class="active-branch-badge">
                        <span class="branch-label">{format!("Branch: {name}")}</span>
                        {render_ci_indicator(&ci_status)}
                        if let Some(ref err) = *branch_error {
                            <span class="branch-bar-error">{err}</span>
                        }
                        <button class="clear-branch-btn" onclick={on_clear_branch.clone()}>
                            {"\u{00D7}"}
                        </button>
                        <div class="branch-badge-actions">
                            <button class={publish_btn_class}
                                    onclick={on_publish}
                                    disabled={*branch_saving || *diff_loading || ci_blocks_publish}>
                                { if *diff_loading { "Loading diff\u{2026}" } else { "Publish" } }
                            </button>
                            <button class="sync-btn"
                                    onclick={on_sync}
                                    disabled={*branch_saving || *syncing}
                                    title={format!("Merge {} into your editor branch", DEFAULT_BRANCH)}>
                                { if *syncing { "Syncing\u{2026}" } else { "Sync from source" } }
                            </button>
                            <button class="discard-btn"
                                    onclick={on_discard}
                                    disabled={*branch_saving}>
                                { if *branch_saving { "Working\u{2026}" } else { "Discard" } }
                            </button>
                        </div>
                    </div>
                    if *show_diff {
                        if let Some(ref data) = *diff_data {
                            {render_diff_panel(data, on_confirm_publish, on_cancel_diff)}
                        }
                    }
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

            // Filter and sort controls
            if !entries.is_empty() || !filter_text.is_empty() {
                <div class="filter-sort-bar">
                    <div class="filter-bar">
                        <input
                            type="text"
                            class="filter-input"
                            placeholder="Search all files\u{2026}"
                            value={(*filter_text).clone()}
                            oninput={on_filter_input}
                        />
                        if !filter_text.is_empty() {
                            <button class="clear-filter-btn" onclick={on_clear_filter}>
                                {"\u{00D7}"}
                            </button>
                        }
                    </div>
                    <div class="sort-bar">
                        <span class="sort-label">{"Sort:"}</span>
                        <button
                            class={classes!("sort-btn", (*sort_mode == SortMode::Alphabetical).then_some("active"))}
                            onclick={on_set_sort_alpha}
                        >
                            {"A\u{2013}Z"}
                        </button>
                        <button
                            class={classes!("sort-btn", (*sort_mode == SortMode::LastModified).then_some("active"))}
                            onclick={on_set_sort_recent}
                        >
                            {"Recent"}
                        </button>
                        if *dates_loading {
                            <span class="loading-dates">{"Loading dates\u{2026}"}</span>
                        }
                    </div>
                    <div class="status-filter-bar">
                        <span class="sort-label">{"Filter:"}</span>
                        <button
                            class={classes!("status-btn", (*status_filter == StatusFilter::All).then_some("active"))}
                            onclick={on_filter_all}
                        >
                            {"All"}
                        </button>
                        <button
                            class={classes!("status-btn", (*status_filter == StatusFilter::Draft).then_some("active"))}
                            onclick={on_filter_draft}
                        >
                            {"\u{1F331} Draft"}
                        </button>
                        <button
                            class={classes!("status-btn", (*status_filter == StatusFilter::Published).then_some("active"))}
                            onclick={on_filter_published}
                        >
                            {"\u{1F4F0} Published"}
                        </button>
                    </div>
                </div>
            }

            if filter_active {
                if *all_files_loading && all_files.is_empty() {
                    <p class="loading">{"Loading index\u{2026}"}</p>
                } else if global_search_results.is_empty() {
                    <p class="empty">{"No files match."}</p>
                } else {
                    <>
                        <p class="search-count">
                            {format!("{} result(s) for \u{2018}{}\u{2019}", global_search_results.len(), *filter_text)}
                        </p>
                        <div class="content-list">
                            { for global_search_results.iter().map(|entry| {
                                let is_media = is_media_file(&entry.name);
                                let onclick = if is_media {
                                    Callback::from(|_: MouseEvent| {})
                                } else {
                                    let path = entry.path.clone();
                                    let on_click = on_navigate_to_file.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        on_click.emit(path.clone());
                                    })
                                };
                                let delete_btn = if is_media && is_authenticated {
                                    let entry_for_delete = entry.clone();
                                    let on_delete = on_delete_request.clone();
                                    let delete_onclick = Callback::from(move |e: MouseEvent| {
                                        e.stop_propagation();
                                        on_delete.emit(entry_for_delete.clone());
                                    });
                                    html! {
                                        <button class="entry-delete-btn"
                                                onclick={delete_onclick}>{"Delete"}</button>
                                    }
                                } else {
                                    html! {}
                                };
                                html! {
                                    <div class={classes!("content-entry", is_media.then_some("is-media"))}
                                         onclick={onclick}>
                                        if is_media {
                                            <span class="entry-icon">{"\u{1F5BC}\u{FE0F}"}</span>
                                        } else {
                                            <span class="entry-icon">{"\u{00B7}"}</span>
                                        }
                                        <span class="entry-name">{&entry.path}</span>
                                        {delete_btn}
                                    </div>
                                }
                            }) }
                        </div>
                    </>
                }
            } else if *loading {
                <p class="loading">{"Loading\u{2026}"}</p>
            } else if let Some(err) = &*error {
                <p class="error">{err}</p>
            } else if entries.is_empty() {
                <p class="empty">{"This directory is empty."}</p>
            } else if display_entries.is_empty() {
                <p class="empty">{"No entries match."}</p>
            } else {
                <div class="content-list">
                    { for display_entries.iter().map(|entry| {
                        render_entry(entry, on_navigate.clone(), &cur_sort, &cur_dates, &cur_post_statuses, &cur_folder_md_statuses, on_delete_request.clone(), is_authenticated)
                    }) }
                </div>
            }

            if let Some(ref entry) = *delete_confirm {
                <div class="modal-overlay" onclick={on_delete_cancel.clone()}>
                    <div class="modal"
                         onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <h3>{"Delete file?"}</h3>
                        <p class="modal-filename">{&entry.name}</p>
                        <p class="modal-warning">
                            {"This will be committed directly to the branch and cannot be undone."}
                        </p>
                        if let Some(ref err) = *delete_error {
                            <p class="error">{err}</p>
                        }
                        <div class="modal-actions">
                            <button onclick={on_delete_cancel.clone()}
                                    disabled={*deleting}>{"Cancel"}</button>
                            <button class="delete-btn"
                                    onclick={on_delete_confirm.clone()}
                                    disabled={*deleting}>
                                { if *deleting { "Deleting\u{2026}" } else { "Delete" } }
                            </button>
                        </div>
                    </div>
                </div>
            }

            if *show_ci_warning_modal {
                <div class="modal-overlay" onclick={on_ci_warning_cancel.clone()}>
                    <div class="modal"
                         onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <h3>{"CI has not passed yet"}</h3>
                        <p class="modal-warning">
                            {"The automated build check has not completed successfully. \
                              Publishing now may deploy a broken build. Are you sure you \
                              want to publish anyway?"}
                        </p>
                        <div class="modal-actions">
                            <button onclick={on_ci_warning_cancel.clone()}>{"Cancel"}</button>
                            <button class="publish-btn pending"
                                    onclick={on_ci_warning_publish.clone()}>
                                {"Publish anyway"}
                            </button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}

// ── Branch list rendering ───────────────────────────────────────

fn render_branch_list(
    branches: &UseStateHandle<Vec<GitRef>>,
    active_branch: &Option<String>,
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
                let is_active = active_branch.as_deref().map_or(false, |b| b == name);
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

fn render_entry(
    entry: &ContentEntry,
    on_click: Callback<ContentEntry>,
    sort_mode: &SortMode,
    commit_dates: &HashMap<String, f64>,
    post_statuses: &HashMap<String, PostStatus>,
    folder_md_statuses: &HashMap<String, PostStatus>,
    on_delete: Callback<ContentEntry>,
    is_authenticated: bool,
) -> Html {
    let is_dir = entry.entry_type == "dir";
    let is_media = !is_dir && is_media_file(&entry.name);

    let onclick = if is_media {
        Callback::from(|_: MouseEvent| {})
    } else {
        let entry_clone = entry.clone();
        Callback::from(move |_: MouseEvent| {
            on_click.emit(entry_clone.clone());
        })
    };

    let date_display = if *sort_mode == SortMode::LastModified {
        commit_dates.get(&entry.path).map(|ms| format_date(*ms))
    } else {
        None
    };

    let status_icon = if !is_dir && entry.name.ends_with(".md") {
        match post_statuses.get(&entry.path) {
            Some(PostStatus::Draft) => {
                html! { <span class="post-status-icon" title="Draft">{ "\u{1F331}" }</span> }
            }
            Some(PostStatus::Published) => {
                html! { <span class="post-status-icon" title="Published">{ "\u{1F4F0}" }</span> }
            }
            _ => html! {},
        }
    } else if is_media {
        html! { <span class="entry-icon">{ "\u{1F5BC}\u{FE0F}" }</span> }
    } else {
        html! {}
    };

    let delete_btn = if is_media && is_authenticated {
        let entry_for_delete = entry.clone();
        let delete_onclick = Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_delete.emit(entry_for_delete.clone());
        });
        html! {
            <button class="entry-delete-btn" onclick={delete_onclick}>{"Delete"}</button>
        }
    } else {
        html! {}
    };

    html! {
        <div class={classes!("content-entry", is_dir.then_some("is-dir"), is_media.then_some("is-media"))}
             onclick={onclick}>
            if is_dir {
                {
                    match folder_md_statuses.get(&entry.path) {
                        Some(PostStatus::Draft) =>
                            html! { <span class="post-status-icon" title="Draft">{ "\u{1F331}" }</span> },
                        Some(PostStatus::Published) =>
                            html! { <span class="post-status-icon" title="Published">{ "\u{1F4F0}" }</span> },
                        _ =>
                            html! { <span class="entry-icon">{ "📂" }</span> },
                    }
                }
            } else {
                {status_icon}
            }
            <span class="entry-name">{ format!("{}{}", &entry.name, if is_dir { "/" } else { "" }) }</span>
            if let Some(ref date) = date_display {
                <span class="entry-date">{date}</span>
            }
            if !is_dir {
                <span class="entry-size">{crate::utils::format_size(entry.size)}</span>
            }
            {delete_btn}
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

fn format_date(epoch_ms: f64) -> String {
    let date = js_sys::Date::new(&JsValue::from_f64(epoch_ms));
    let year = date.get_full_year();
    let month = date.get_month() + 1;
    let day = date.get_date();
    format!("{year}-{month:02}-{day:02}")
}

// ── CI status ────────────────────────────────────────────────────

async fn fetch_ci_status(token: &str, branch: &str, ci_status: &UseStateHandle<CiState>) {
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
            let failure = resp
                .check_runs
                .iter()
                .find(|r| r.conclusion.as_deref() == Some("failure"));
            if let Some(failed) = failure {
                ci_status.set(CiState::Failure(failed.html_url.clone()));
            } else {
                ci_status.set(CiState::Success);
            }
        }
        Err(_) => {}
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

// ── Diff panel ───────────────────────────────────────────────────

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
