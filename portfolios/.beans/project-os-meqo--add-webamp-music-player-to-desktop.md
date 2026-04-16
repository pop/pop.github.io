---
# project-os-meqo
title: Add Webamp music player to desktop
status: completed
type: feature
priority: normal
created_at: 2026-04-16T19:04:04Z
updated_at: 2026-04-16T19:07:54Z
---

Mount Webamp (https://github.com/captbaritone/webamp) as an always-on fixture in the Win95 desktop. Acts like the Clippy widget — instantiated once at app init, not launched from an icon (Option B from the investigation).

## Requirements

- **No autoplay.** Webamp must load paused; user must click play.
- **Configurable playlist** in `portfolios.toml` under a new `[webamp]` (or `[[webamp.tracks]]`) section. Schema should support `url`, `artist`, `title` per track.
- **Lazy-load via CDN.** Use `https://unpkg.com/webamp@^2` rather than vendoring/npm. Don't penalize first paint — load the script tag in `index.html`.
- **Mounted alongside Clippy** in `src/app.rs` (component tree fixture, not a Window).
- **Optional/disableable.** If `[webamp]` section is missing or has no tracks, do not render Webamp at all.
- **License:** MIT, attribution-friendly — add a line to the Clippy "About this Portfolio" modal in `portfolios.toml` crediting Webamp.

## Implementation Sketch

1. **`index.html`** — add `<script type="module">` that imports Webamp from unpkg and stashes it on `window.Webamp` so wasm-bindgen can reach it.
2. **`Cargo.toml`** — add `serde-wasm-bindgen` for converting Rust structs → JS options object. Add `web-sys` features if needed (likely `Element` already covers it).
3. **`src/config.rs`** — extend `Config` with `webamp: Option<WebampConfig>` where `WebampConfig { tracks: Vec<WebampTrack> }` and `WebampTrack { url, artist, title }`.
4. **`src/components/webamp.rs`** — new component:
   - `wasm-bindgen extern` block declaring `Webamp` constructor, `renderWhenReady`, `close`.
   - Yew `function_component` that takes `tracks: Vec<WebampTrack>` as props.
   - `use_effect_with` on mount: build options JS object via `serde_wasm_bindgen::to_value`, call `new Webamp(opts)`, then `render_when_ready(container_ref)`.
   - Cleanup closure: call `webamp.close()` to tear down DOM/audio nodes.
   - Container `<div id="webamp-mount">` with `position: fixed` somewhere unobtrusive (top-right? bottom-left above taskbar?).
5. **`src/components/mod.rs`** — register module.
6. **`src/app.rs`** — render `<Webamp tracks=... />` alongside `<Clippy />`, conditional on config presence.
7. **`portfolios.toml`** — add a `[[webamp.tracks]]` entry with at least one track. (User needs to provide an actual MP3 — placeholder URL is fine for the PR; user can swap it in later.)
8. **`portfolios.toml`** — add a Webamp credit line to `clippy.modal_paragraphs`.
9. **z-index sanity check** — Webamp uses its own absolute positioning. Confirm it doesn't fight with `WindowManager.z_counter` or sit under the fixed taskbar.

## Acceptance

- [ ] `[[webamp.tracks]]` section in `portfolios.toml` is honored
- [ ] Player visible on page load, NOT auto-playing
- [ ] Clicking play in Webamp's UI plays the configured track
- [ ] Removing the `[webamp]` section hides the player entirely (no JS errors)
- [ ] Webamp doesn't block first paint (CDN script loads async/deferred)
- [ ] `cargo check --target wasm32-unknown-unknown` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown` passes
- [ ] Credit line added to Clippy "About" modal

## Todo

- [x] Add CDN script tag to `index.html`
- [x] Extend `Config` with optional `WebampConfig`
- [x] Create `src/components/webamp.rs` with wasm-bindgen externs + Yew component
- [x] Register component in `mod.rs`
- [x] Mount conditionally in `src/app.rs`
- [x] Add `[[webamp.tracks]]` placeholder + Webamp credit to `portfolios.toml`
- [x] Verify cargo check + clippy pass
- [ ] Manual: `trunk serve`, confirm player appears, doesn't autoplay, plays on click

## Summary of Changes

- **`index.html`**: Added a `<script type="module">` that imports Webamp from `unpkg.com/webamp@^2`, stashes it on `window.Webamp`, and dispatches a `webamp-ready` CustomEvent.
- **`Cargo.toml`**: Added `serde-wasm-bindgen = "0.6"` for Rust → JS options object conversion.
- **`src/config.rs`**: Added `WebampConfig { tracks: Vec<WebampTrack> }` and `WebampTrack { url, artist, title }`; `Config.webamp` is `Option<WebampConfig>` so the section is fully optional.
- **`src/components/webamp.rs`** (new): Yew `function_component` with a `wasm_bindgen extern "C"` block for Webamp's constructor, `renderWhenReady`, and `close`. Uses `use_effect_with` to construct the player on mount, waits for the async CDN load via `webamp-ready` event (with a 100ms Timeout fallback), and calls `close()` in the cleanup closure.
- **`src/components/mod.rs`**: Registered `pub mod webamp`.
- **`src/app.rs`**: Conditionally renders `<Webamp />` alongside `<Clippy />` only when `config.webamp` is `Some` with non-empty tracks.
- **`portfolios.toml`**: Added a `[webamp]` section with a single placeholder track (`/music/placeholder.mp3`); the user needs to provide a real MP3. Added a Webamp MIT credit line to `clippy.modal_paragraphs`.
- **`src/components/taskbar.rs`**: Fixed a pre-existing `clippy::redundant_closure` warning (`|| current_time()` → `current_time`) so `clippy -- -D warnings` passes cleanly.

## Notes

- No autoplay: Webamp loads paused by default; browser autoplay policies also block sound without user gesture.
- Webamp manages its own draggable chrome — it is mounted in a plain `<div id="webamp-mount">` rather than wrapped in our `<Window>`.
- Removing or emptying `[webamp]` in the config hides the player; no JS/WASM error paths.
- Manual `trunk serve` verification is left unchecked on the todo list — it requires a browser and is outside this automated flow.

## Validation

- `cargo check --target wasm32-unknown-unknown`: clean
- `cargo clippy --target wasm32-unknown-unknown -- -D warnings`: clean
