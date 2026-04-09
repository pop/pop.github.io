# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Win95-themed games portfolio — a Rust/WASM web app styled like a Windows 95 desktop with draggable windows, a taskbar, Start menu, and Clippy widget.

## Commands

```bash
trunk serve                                  # dev server (hot reload)
trunk build                                  # production WASM bundle → dist/
cargo check --target wasm32-unknown-unknown  # fast compile check
cargo clippy --target wasm32-unknown-unknown # lint
```

Trunk automatically handles WASM compilation and asset bundling. Output goes to `dist/`.

## Architecture

**Framework**: [Yew](https://yew.rs/) — React-like component tree compiled to WASM.

**Entry point**: `src/main.rs` initializes `wasm_logger` then calls `Renderer::new().render()` with the root `App` component.

**State**: `src/state.rs` — `WindowManager` owns all window state: open/closed, absolute position, and a monotonically increasing `z_counter` for focus ordering. Each `WindowState` stores `game_id`, `pos: (i32, i32)`, and `z_index`.

**Config**: `src/config.rs` — Game entries and Clippy quotes loaded at compile time from `games.toml` via `include_str!` + serde. No runtime file I/O.

**Component tree**:
```
App
├── Desktop (position:relative, overflow:hidden — clips windows to viewport)
│   ├── GameIcon × N  (clickable icons, open a window on click)
│   └── Window × N    (draggable Win95 shell, title bar + close button)
│       └── GameWindow (game overview content: screenshot, description, links)
├── Taskbar (fixed bottom — open window buttons, live clock, Start button)
│   └── StartMenu (popup, shown on Start button click)
└── Clippy (fixed corner — rotating quotes from config, AI disclosure modal)
```

**Dragging** (in `window.rs`): `onmousedown` on the title bar records offset; document-level `gloo_events::EventListener` on `mousemove`/`mouseup` updates position. Touch equivalents use `touchstart`/`touchmove`/`touchend`.

**Styling**: `win95.css` loaded from CDN for authentic Win95 chrome. Custom layout in `styles/main.css`. Window positions set via `inline style` (`position:absolute; left:Xpx; top:Ypx; z-index:N`).

## Tickets & Commits

Tickets live in `.beans/` (prefix `project-os-`). Run `beans prime` for full agent instructions.

All commits must reference the ticket being resolved, e.g.:
```
feat(project-os): draggable window shell (project-os-jcgo)
```

Mark tickets `done` when committed.

## Key Dependencies

| Crate | Purpose |
|---|---|
| `yew` | Component framework |
| `wasm-bindgen` | Rust ↔ JS bindings |
| `gloo-timers` | `setInterval` for Clippy quote rotation |
| `gloo-events` | Document-level event listeners for drag |
| `web-sys` / `js-sys` | DOM/JS APIs |
| `serde` + `toml` | Compile-time config deserialization |
