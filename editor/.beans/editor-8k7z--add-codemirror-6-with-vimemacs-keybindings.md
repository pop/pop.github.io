---
# editor-8k7z
title: Add CodeMirror 6 with vim/emacs keybindings
status: completed
type: feature
priority: normal
created_at: 2026-07-12T04:05:46Z
updated_at: 2026-07-12T04:17:28Z
---

Replace the plain <textarea> editor with CodeMirror 6 (+ @replit/codemirror-vim). Goals: vim mode (normal/insert/visual/ex), emacs-style line nav (ctrl+a, ctrl+e, ctrl+k), syntax highlighting for markdown. Integration via JS module loaded from esm.sh CDN, called from Rust via wasm_bindgen externs.

## Summary of Changes

- Added `js/codemirror.js` — ES module loaded from esm.sh CDN, exposes `window.cm*` API (create/set/get/insert/wrap/destroy/focus)
- Uses `?deps=` pinning on all esm.sh imports to deduplicate shared `@codemirror/state`/`@codemirror/view` instances (prevents silent vim mode breakage)
- `window.cmIsReady` flag + `cm-ready` event dispatched at end of module for race detection
- Updated `index.html` to copy-file the JS module via Trunk and load it as `<script type="module">`
- Replaced `<textarea>` with a `<div ref={cm_mount_ref}>` that is always in the DOM; hidden via `.cm-editor-hidden` in preview mode to preserve cursor state
- Added `wasm_bindgen` externs for all `window.cm*` functions
- Added `vim_mode: UseStateHandle<bool>` state + Vim toggle button in toolbar
- Mount effect `use_effect_with((loading, vim_mode), ...)` creates/recreates CM6 on load or mode change; cleanup destroys it
- `externally_setting: Rc<RefCell<bool>>` flag prevents feedback loops when Rust calls `cmSetValue` (e.g. on revert/discard)
- `sync_to_cm()` helper called in all external content-change paths (revert effect, cancel revert, discard-then-revert)
- Format toolbar macros use `cmWrapSelection` instead of `apply_format_to_content`
- Image upload uses `cmInsertAtCursor` instead of textarea selection APIs
- `emacsStyleKeymap` only included when vim mode is OFF (prevents conflicts with vim's own Ctrl-A/E/K bindings)
- Added CM6 CSS: mount container sizing, split-mode heights, vim status bar, vim toggle button styles
- Removed `apply_format_to_content` function and `HtmlTextAreaElement` import
