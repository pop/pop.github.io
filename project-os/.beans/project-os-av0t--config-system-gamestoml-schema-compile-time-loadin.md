---
# project-os-av0t
title: 'config system: games.toml schema + compile-time loading'
status: todo
type: task
priority: high
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Define the config schema and embed it at compile time.
- games.toml with: [[games]] entries (id, title, description, contributors, tech[{name,icon}], icon, demo, launch_url, launch_type), [[quotes]] entries, [start_menu] (about_url, github_url, itchio_url)
- src/config.rs: serde-deserializable structs + load_config() using include_str!("../games.toml")
- src/state.rs: WindowManager struct (windows: Vec<WindowState>, z_counter: u32) + WindowState (game_id, open, pos: (i32,i32), z_index)
- Add at least 1-2 placeholder game entries and 3+ clippy quotes
- Verify: cargo check passes
