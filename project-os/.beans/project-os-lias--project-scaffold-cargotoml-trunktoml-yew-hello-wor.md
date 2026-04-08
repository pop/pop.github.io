---
# project-os-lias
title: 'project scaffold: Cargo.toml, Trunk.toml, Yew hello world compiles to WASM'
status: completed
type: task
priority: high
created_at: 2026-04-08T04:55:30Z
updated_at: 2026-04-08T06:03:33Z
parent: project-os-zsw7
---

Set up the standalone Rust project in project-os/. Includes:
- Cargo.toml with yew, wasm-bindgen, serde, toml, gloo-timers, js-sys, web-sys deps
- Trunk.toml (target index.html, dist dist/)
- src/main.rs: wasm_logger init + Renderer::new().render()
- src/app.rs: minimal App component returning html!{ <p>"Hello"</p> }
- Verify: trunk build succeeds, trunk serve shows hello world

## Summary of Changes

- Added `Cargo.toml` with all required dependencies: yew 0.21 (csr), wasm-bindgen, serde (derive), toml 0.8, gloo-timers (futures), gloo-events, js-sys, web-sys, wasm-logger, log
- Added `Trunk.toml` with target index.html and dist/ output directory
- Added `index.html` minimal HTML page for Trunk injection
- Added `src/main.rs` with wasm_logger init and `yew::Renderer::<App>::new().render()`
- Added `src/app.rs` with minimal App function component returning `html!{ <p>{"Hello"}</p> }`
- Verified: `cargo check --target wasm32-unknown-unknown` passes successfully
