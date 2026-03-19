---
# editor-x78j
title: CSS — styles for history panel and revert banners
status: completed
type: task
priority: normal
created_at: 2026-03-19T03:36:18Z
updated_at: 2026-03-19T21:10:03Z
parent: editor-eb70
blocked_by:
    - editor-67fw
    - editor-0fro
---

## What

Add CSS styles for the version history feature to `styles/main.css`. All new classes are used by editor.rs (T5 and T6).

## Context — existing CSS conventions

The stylesheet already has a `/* Branch selector */` section (around line 1083) with classes like `.branch-toggle-btn`, `.branch-list`, `.branch-list-header`, `.branch-item`. The history panel should follow the same visual style — border, border-radius, spacing, typography — since it's a similar collapsible widget.

The editor has a dark toolbar (`background: #2d2d2d`, white text) and a light content area. The history panel sits between the toolbar and editor, styled similarly to `.branch-list`.

## Classes to add

Add a `/* Version history */` section after the `/* Branch selector */` section in `styles/main.css`.

### History toggle button

```css
/* Version history */

.history-toggle-btn {
    padding: 0.4rem 0.8rem;
    border: 1px solid #0366d6;
    border-radius: 4px;
    background: transparent;
    color: #0366d6;
    cursor: pointer;
    font-size: 0.85rem;
}

.history-toggle-btn:hover:not(:disabled) {
    background: #0366d6;
    color: white;
}

.history-toggle-btn:disabled {
    opacity: 0.45;
    cursor: default;
}
```

### History panel

```css
.history-panel {
    border: 1px solid #e1e4e8;
    border-radius: 6px;
    overflow: hidden;
    margin-bottom: 0.5rem;
    background: #fff;
}

.history-panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    background: #f6f8fa;
    border-bottom: 1px solid #e1e4e8;
    font-size: 0.85rem;
}

.history-panel-title {
    font-weight: 600;
    color: #333;
}

.history-loading,
.history-empty {
    padding: 0.75rem;
    color: #586069;
    font-size: 0.85rem;
    margin: 0;
}

.history-list {
    max-height: 280px;
    overflow-y: auto;
}

.history-item {
    padding: 0.5rem 0.75rem;
    cursor: pointer;
    border-bottom: 1px solid #f0f0f0;
    transition: background 0.1s;
}

.history-item:last-child {
    border-bottom: none;
}

.history-item:hover {
    background: #f6f8fa;
}

.history-item-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.2rem;
}

.history-item-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
}

.history-sha {
    font-family: monospace;
    font-size: 0.78rem;
    color: #0366d6;
    background: #f1f8ff;
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
}

.history-date {
    font-size: 0.78rem;
    color: #586069;
}

.history-msg {
    font-size: 0.82rem;
    color: #333;
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.history-stats {
    display: flex;
    gap: 0.3rem;
    font-size: 0.78rem;
    font-weight: 600;
    flex-shrink: 0;
}

.history-add {
    color: #22863a;
}

.history-del {
    color: #cb2431;
}
```

### Unsaved-changes gate banner

Rendered inside the history panel (below the commit list) when the user selects a commit but has unsaved changes:

```css
.revert-gate-banner {
    padding: 0.75rem;
    background: #fffbdd;
    border-top: 1px solid #e1e4e8;
    font-size: 0.85rem;
}

.revert-gate-banner p {
    margin: 0 0 0.5rem 0;
    color: #735c0f;
}

.revert-gate-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
}

.discard-btn {
    padding: 0.35rem 0.7rem;
    border: 1px solid #cb2431;
    border-radius: 4px;
    background: transparent;
    color: #cb2431;
    cursor: pointer;
    font-size: 0.82rem;
}

.discard-btn:hover:not(:disabled) {
    background: #cb2431;
    color: white;
}

.discard-btn:disabled {
    opacity: 0.45;
    cursor: default;
}
```

### Revert preview banner

Shown above the editor content area while in history preview mode:

```css
.revert-preview-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.6rem 0.75rem;
    background: #dbedff;
    border: 1px solid #0366d6;
    border-radius: 4px;
    margin-bottom: 0.5rem;
    flex-wrap: wrap;
}

.revert-preview-msg {
    font-size: 0.85rem;
    color: #0550ae;
    flex: 1;
}

.revert-preview-actions {
    display: flex;
    gap: 0.5rem;
}

.confirm-revert-btn {
    padding: 0.35rem 0.7rem;
    border: 1px solid #22863a;
    border-radius: 4px;
    background: #22863a;
    color: white;
    cursor: pointer;
    font-size: 0.82rem;
}

.confirm-revert-btn:hover:not(:disabled) {
    background: #176931;
    border-color: #176931;
}

.confirm-revert-btn:disabled {
    opacity: 0.45;
    cursor: default;
}

.cancel-revert-btn {
    padding: 0.35rem 0.7rem;
    border: 1px solid #cb2431;
    border-radius: 4px;
    background: transparent;
    color: #cb2431;
    cursor: pointer;
    font-size: 0.82rem;
}

.cancel-revert-btn:hover:not(:disabled) {
    background: #cb2431;
    color: white;
}

.cancel-revert-btn:disabled {
    opacity: 0.45;
    cursor: default;
}
```

## Placement in file

Add the entire `/* Version history */` section after the last line of the `/* Branch selector */` section (currently ending around line 1200 with `.branch-item.active`). Do not modify any existing CSS.

## Files

- `styles/main.css` — only file to touch

## Todo

- [x] Add `/* Version history */` section to `styles/main.css`
- [x] Add `.history-toggle-btn` and its states
- [x] Add `.history-panel`, `.history-panel-header`, `.history-panel-title`
- [x] Add `.history-loading`, `.history-empty`, `.history-list`
- [x] Add `.history-item` and its sub-elements (`.history-item-top`, `.history-item-bottom`, `.history-sha`, `.history-date`, `.history-msg`, `.history-stats`, `.history-add`, `.history-del`)
- [x] Add `.revert-gate-banner`, `.revert-gate-actions`, `.discard-btn`
- [x] Add `.revert-preview-banner`, `.revert-preview-msg`, `.revert-preview-actions`, `.confirm-revert-btn`, `.cancel-revert-btn`

## Summary of Changes

Added `/* Version history */` section to `styles/main.css` after the branch selector section, covering all history panel, gate banner, and revert preview banner classes.
