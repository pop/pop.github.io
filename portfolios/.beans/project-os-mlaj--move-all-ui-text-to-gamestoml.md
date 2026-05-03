---
# project-os-mlaj
title: Move all UI text to games.toml
status: completed
type: task
priority: normal
created_at: 2026-04-09T04:08:47Z
updated_at: 2026-04-09T04:21:20Z
---

Move hardcoded text from .rs files into games.toml so all UI text is editable without touching Rust: Clippy modal (title, paragraphs, ok button, icon), start menu labels and shutdown message, taskbar start button label, game launch label.

## Summary of Changes

- Added  section: icon, modal_title, modal_ok_label, modal_paragraphs
- Added  section: start_label
- Extended : about_label, github_label, itchio_label, shutdown_label, shutdown_message
- Added  field to each  entry
- Updated config.rs: new ClippyConfig and TaskbarConfig structs, extended StartMenu
- Updated all components to read from config instead of hardcoded strings
- Compiles cleanly with cargo check --target wasm32-unknown-unknown
