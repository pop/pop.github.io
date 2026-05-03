---
# project-os-av0t
title: 'config system: games.toml schema + compile-time loading'
status: completed
type: task
priority: high
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T17:30:44Z
parent: project-os-zsw7
---

Define the config schema and embed it at compile time.
- games.toml with: [[games]] entries (id, title, description, contributors, tech[{name,icon}], icon, demo, launch_url, launch_type), [[quotes]] entries, [start_menu] (about_url, github_url, itchio_url)
- src/config.rs: serde-deserializable structs + load_config() using include_str!("../games.toml")
- src/state.rs: WindowManager struct (windows: Vec<WindowState>, z_counter: u32) + WindowState (game_id, open, pos: (i32,i32), z_index)
- Add at least 1-2 placeholder game entries and 3+ clippy quotes
- Verify: cargo check passes

Implemented: games.toml with 2 placeholder games (snake, tetris), 4 Clippy quotes, and start_menu config. src/config.rs with serde-deserializable structs and load_config(). src/state.rs with WindowManager and WindowState structs. Both modules wired into src/main.rs. cargo check --target wasm32-unknown-unknown passes with only expected dead_code warnings.
