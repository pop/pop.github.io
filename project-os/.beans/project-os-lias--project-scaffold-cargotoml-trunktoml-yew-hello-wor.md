---
# project-os-lias
title: 'project scaffold: Cargo.toml, Trunk.toml, Yew hello world compiles to WASM'
status: todo
type: task
priority: high
created_at: 2026-04-08T04:55:30Z
updated_at: 2026-04-08T04:55:30Z
parent: project-os-zsw7
---

Set up the standalone Rust project in project-os/. Includes:
- Cargo.toml with yew, wasm-bindgen, serde, toml, gloo-timers, js-sys, web-sys deps
- Trunk.toml (target index.html, dist dist/)
- src/main.rs: wasm_logger init + Renderer::new().render()
- src/app.rs: minimal App component returning html!{ <p>"Hello"</p> }
- Verify: trunk build succeeds, trunk serve shows hello world
