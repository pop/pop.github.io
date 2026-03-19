---
# editor-0fro
title: Editor — revert flow (select, commit, preview mode, confirm/cancel)
status: completed
type: feature
priority: high
created_at: 2026-03-19T03:34:43Z
updated_at: 2026-03-19T21:08:38Z
parent: editor-eb70
blocked_by:
    - editor-gtln
    - editor-67fw
---

## What

Implement the revert action in `src/components/editor.rs`. When the user clicks a commit in the history panel, this flow:

1. Checks for unsaved changes — forces save or discard first
2. Creates the revert commits on the branch via `revert_directory_to_commit`
3. Reloads the post content from the branch HEAD
4. Enters "history preview mode" — shows a banner with Confirm and Cancel buttons
5. **Confirm**: exits preview mode (revert commits stay)
6. **Cancel**: calls `revert_directory_to_commit` again with the pre-revert branch HEAD SHA to restore the prior state

## New state variables

```rust
let reverting = use_state(|| false);
// When Some(sha), we are in history preview mode.
// The sha is the branch HEAD SHA *before* the revert — used for Cancel.
let history_preview_pre_sha = use_state(|| Option::<String>::None);
// When Some(sha), a commit was selected but we're waiting for save/discard first
let pending_revert_sha = use_state(|| Option::<String>::None);
```

## on_history_select callback (replaces T5 no-op)

```rust
let on_history_select = {
    let pending_revert_sha = pending_revert_sha.clone();
    let content = content.clone();
    let original_content = original_content.clone();
    let is_new = is_new.clone();
    Callback::from(move |sha: String| {
        // Always set pending — the revert effect guards on no unsaved changes
        pending_revert_sha.set(Some(sha));
    })
};
```

## Unsaved-changes gate banner

Rendered inside the history panel area (below the commit list), when `pending_revert_sha.is_some()` AND `has_changes` is true:

```rust
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
```

Callbacks:
- `on_cancel_pending_revert`: `pending_revert_sha.set(None)`
- `on_discard_then_revert`: reload content/original_content from branch HEAD (call `client.get_file`), then the revert effect fires because `original_content == content`
- `on_save_then_revert`: same as `on_save`, but `pending_revert_sha` stays set; after save completes and `original_content` is updated, the revert effect below fires

## Revert effect

`use_effect_with` on `((*pending_revert_sha).clone(), (*original_content).clone())`. Fires when `pending_revert_sha` is `Some` AND content is clean (no unsaved changes):

```rust
use_effect_with(
    ((*pending_revert_sha).clone(), (*original_content).clone()),
    move |(pending_sha, _)| {
        let Some(sha) = pending_sha.clone() else { return || (); };
        if *content != *original_content || *is_new {
            return || (); // still has unsaved changes — wait
        }

        // spawn revert
        wasm_bindgen_futures::spawn_local(async move {
            let Some(token) = token else { return; };
            let Some(branch) = active_branch else { return; };
            let client = GitHubClient::new(token);

            let pre_sha = match client.get_branch_sha(&branch).await {
                Ok(s) => s,
                Err(e) => { set_error(e); return; }
            };

            reverting.set(true);

            match client.revert_directory_to_commit(&path, &sha, &branch).await {
                Ok(new_file_sha) => {
                    match client.get_file(&path, &branch).await {
                        Ok(f) => {
                            let text = f.content.unwrap_or_default();
                            content.set(text.clone());
                            original_content.set(text);
                            file_sha.set(Some(new_file_sha));
                        }
                        Err(e) => { set_error(e); reverting.set(false); return; }
                    }
                    history_preview_pre_sha.set(Some(pre_sha));
                    pending_revert_sha.set(None);
                    view_mode.set(ViewMode::Split);
                    history_commits.set(vec![]); // invalidate so panel refreshes next open
                    reverting.set(false);
                }
                Err(e) => {
                    set_error(e);
                    pending_revert_sha.set(None);
                    reverting.set(false);
                }
            }
        });

        || ()
    },
);
```

## Preview mode banner

Shown in the view above the editor container when `history_preview_pre_sha.is_some()`:

```rust
if history_preview_pre_sha.is_some() {
    <div class="revert-preview-banner">
        <span class="revert-preview-msg">
            {"Previewing restored version — confirm or cancel."}
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
```

## Confirm callback

```rust
let on_confirm_revert = {
    let history_preview_pre_sha = history_preview_pre_sha.clone();
    let save_msg = save_msg.clone();
    Callback::from(move |_: MouseEvent| {
        history_preview_pre_sha.set(None);
        save_msg.set(Some("Version restored".into()));
    })
};
```

## Cancel callback

Calls `revert_directory_to_commit` with the pre-revert SHA to undo the revert:

```rust
let on_cancel_revert = {
    // clone: history_preview_pre_sha, reverting, content, original_content,
    //        file_sha, set_error, token, path, active_branch
    Callback::from(move |_: MouseEvent| {
        let Some(pre_sha) = (*history_preview_pre_sha).clone() else { return; };
        if let (Some(token), Some(branch)) = (token.clone(), active_branch.clone()) {
            reverting.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let client = GitHubClient::new(token);
                match client.revert_directory_to_commit(&path, &pre_sha, &branch).await {
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
                    Err(e) => { set_error(e); reverting.set(false); }
                }
            });
        }
    })
};
```

## Toolbar constraints in preview mode

- Save button: add `|| history_preview_pre_sha.is_some()` to its `disabled` condition
- History button: add `|| history_preview_pre_sha.is_some()` to its `disabled` condition

## Files

- `src/components/editor.rs` — all changes here
- `src/services/github.rs` — already has `revert_directory_to_commit` from T4

## Validation

```bash
cargo check --target wasm32-unknown-unknown
cargo clippy
```

## Todo

- [x] Add `reverting`, `history_preview_pre_sha`, `pending_revert_sha` state vars
- [x] Implement `on_history_select` (replaces T5 no-op)
- [x] Implement unsaved-changes gate banner with Save/Discard/Cancel
- [x] Implement `on_cancel_pending_revert`, `on_discard_then_revert`, `on_save_then_revert` callbacks
- [x] Implement revert effect watching `(pending_revert_sha, original_content)`
- [x] Render preview mode banner above the editor container
- [x] Implement `on_confirm_revert` and `on_cancel_revert` callbacks
- [x] Disable Save and History buttons while in preview mode
- [x] Validate with `cargo check --target wasm32-unknown-unknown`

## Summary of Changes

Implemented full revert flow in `src/components/editor.rs`: history select sets pending SHA, unsaved-changes gate forces save/discard first, revert effect fires when content is clean, preview mode shows confirm/cancel banner, cancel revert restores pre-revert state.
