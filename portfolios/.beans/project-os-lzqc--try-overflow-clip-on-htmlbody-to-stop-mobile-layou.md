---
# project-os-lzqc
title: 'Try overflow: clip on html/body to stop mobile layout-viewport re-resolution'
status: completed
type: bug
priority: high
created_at: 2026-04-19T23:13:46Z
updated_at: 2026-04-19T23:52:08Z
---

## Problem

On mobile (Chrome Android + Safari iOS), dragging the Webamp player's title bar past the right edge of the viewport causes the page's fixed taskbar and Clippy widget to progressively shift out of the visual viewport. Root cause: Webamp appends `#webamp` to `document.body` and clamps drag to `max(scrollWidth, offsetWidth)` of the body. Dragging past the right edge grows `body.scrollWidth`, which makes mobile Chrome re-resolve the layout viewport width. `position: fixed` elements (taskbar, Clippy) are anchored to the layout viewport and visually displace.

Prior attempts that failed: Solution A (`.webamp-stage { position: fixed; inset: 0 }`) broke Webamp interactivity because Webamp's drag math uses `getBoundingClientRect` against the normal offset chain, which a `position: fixed` ancestor disrupts. Solution B (`MutationObserver` clamping inline `style.left`/`top`) didn't help because by the time we clamp, `body.scrollWidth` has already grown.

## Approach (Solution C — cheap smoke test)

Change `styles/main.css` to use `overflow: clip` on `html, body` instead of `overflow: hidden`. Unlike `hidden`, `clip` does not create a scroll container, so it cannot contribute a scroll origin. Theory: mobile Chrome's layout-viewport re-resolution algorithm may not count the Webamp overflow as scrollable when the root uses `clip`, preventing the width expansion that displaces fixed elements.

## Files

- `styles/main.css` — change `overflow: hidden` on the `html, body` block to `overflow: clip`. Optionally add `contain: layout` to `#desktop`.

## Todo

- [x] Change `overflow: hidden` → `overflow: clip` on `html, body` in `styles/main.css`
- [x] `cargo check --target wasm32-unknown-unknown` passes
- [x] `cargo clippy --target wasm32-unknown-unknown` passes with no new warnings

## Trade-offs

- `overflow: clip` is Baseline 2023 (Chrome 90+, Safari 16+, Firefox 81+) — safe for our target audience.
- Whether the root `overflow: clip` prevents layout-viewport re-resolution in mobile Chrome is not spec-guaranteed. Cheap to try; may or may not work.
- If it works, ships nothing but a 1-line CSS change. If not, fall through to `renderInto()` approach (sibling ticket).


## Summary of Changes

Changed `overflow: hidden` to `overflow: clip` on the `html, body` rule in `styles/main.css` (Solution C smoke test). Unlike `hidden`, `clip` does not create a scroll container, so it cannot contribute a scroll origin that mobile Chrome's layout-viewport re-resolution algorithm could key off of. No other changes — `contain: layout` on `#desktop` intentionally skipped to keep the diff minimal. `cargo check` and `cargo clippy -- -D warnings` both pass.
