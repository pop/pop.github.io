---
# editor-67fw
title: Editor — history panel UI (display only)
status: completed
type: feature
priority: high
created_at: 2026-03-19T03:31:03Z
updated_at: 2026-03-19T21:04:34Z
parent: editor-eb70
blocked_by:
    - editor-nsws
    - editor-bqzl
---

## What

Add a collapsible history panel to the editor toolbar in `src/components/editor.rs`. This ticket covers display only — fetching and rendering the commit list. The click-to-revert action is handled in T6.

## New state variables to add

Add inside `editor_page()`, alongside the other `use_state` declarations:

```rust
let show_history = use_state(|| false);
let history_commits = use_state(|| Vec::<CommitSummary>::new());
let history_loading = use_state(|| false);
let history_error = use_state(|| Option::<String>::None);
```

## New imports to add

```rust
use crate::models::github::CommitSummary;
```

## History fetch effect

When `show_history` becomes true and `history_commits` is empty (i.e., first open), fetch the commit list:

```rust
{
    let history_commits = history_commits.clone();
    let history_loading = history_loading.clone();
    let history_error = history_error.clone();
    let token = auth.token.clone();
    let path = props.path.clone();
    let active_branch = auth.active_branch.clone();
    let show_history_val = *show_history;

    use_effect_with(
        (show_history_val, active_branch.clone()),
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
```

## Toggle callback

```rust
let toggle_history = {
    let show_history = show_history.clone();
    Callback::from(move |_: MouseEvent| {
        show_history.set(!*show_history);
    })
};
```

## Toolbar button placement

In the toolbar `html!` block, add the History button **between the Save button and the Delete button**:

Current toolbar order: Save | Delete | Upload Image | [format buttons] | [view toggle]
New toolbar order: Save | **History** | Delete | Upload Image | [format buttons] | [view toggle]

```rust
// History button — only shown when authenticated and on an editor branch
if is_authenticated {
    if auth.active_branch.is_some() {
        if file_sha.is_some() {
            <button
                class="history-toggle-btn"
                onclick={toggle_history.clone()}
                disabled={*saving || *history_loading}
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
}
```

## History panel rendering

Add a `render_history_panel` free function (following the same pattern as dashboard's `render_branch_list`):

```rust
fn render_history_panel(
    commits: &[CommitSummary],
    loading: bool,
    error: &Option<String>,
    on_select: Callback<String>,  // emits commit SHA
) -> Html {
    html! {
        <div class="history-panel">
            <div class="history-panel-header">
                <span class="history-panel-title">{"Post history"}</span>
            </div>
            if loading {
                <p class="history-loading">{"Loading history\u2026"}</p>
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
                        let short_sha = &c.sha[..7];
                        let short_msg = if c.message.len() > 60 {
                            format!("{}\u2026", &c.message[..60])
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
```

Add a date-formatting helper:

```rust
/// Format an ISO 8601 date string for display in the history panel.
/// Returns e.g. "2026-03-17 10:00" (UTC).
fn format_history_date(iso: &str) -> String {
    // Use js_sys::Date to parse and format
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
```

## Wire up panel in the view

In the main `html!` block, after the toolbar `</div>` and before the editor container:

```rust
if *show_history {
    {render_history_panel(
        &history_commits,
        *history_loading,
        &history_error,
        on_history_select.clone(),  // T6 will implement this callback; for now use a no-op
    )}
}
```

For this ticket, `on_history_select` can be a no-op callback:

```rust
let on_history_select = Callback::from(move |_sha: String| {
    // T6: implement revert flow here
});
```

## Files

- `src/components/editor.rs` — all changes in this ticket
- `src/models/github.rs` — already modified by T1 (just add the import)

## Validation

```bash
cargo check --target wasm32-unknown-unknown
cargo clippy
```

No unused variable warnings. If `on_history_select` creates a clippy warning due to the unused `_sha` parameter, name it `_sha` (leading underscore suppresses the warning without `#[allow(dead_code)]`).

## Todo

- [x] Add `show_history`, `history_commits`, `history_loading`, `history_error` state vars
- [x] Add `CommitSummary` import
- [x] Add history fetch effect
- [x] Add `toggle_history` callback
- [x] Add History button to toolbar (between Save and Delete)
- [x] Add `render_history_panel` free function
- [x] Add `format_history_date` helper function
- [x] Wire panel into the view with a no-op `on_history_select`
- [x] Validate with `cargo check --target wasm32-unknown-unknown`

## Summary of Changes

Added history panel UI to `src/components/editor.rs`: state vars, toggle callback, fetch effect, History toolbar button, `render_history_panel` free function, and `format_history_date` helper. Panel shows commit list with SHA, date, message, and +/- stats.
