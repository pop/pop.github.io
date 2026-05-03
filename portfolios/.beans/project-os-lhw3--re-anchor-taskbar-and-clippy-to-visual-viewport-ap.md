---
# project-os-lhw3
title: Re-anchor taskbar and Clippy to Visual Viewport API
status: scrapped
type: bug
priority: normal
created_at: 2026-04-19T23:14:21Z
updated_at: 2026-04-20T02:40:43Z
---

## Problem

Same root cause as project-os-lzqc / project-os-ym2f: Webamp's drag grows the body width on mobile, triggering layout-viewport re-resolution. Fixed taskbar and Clippy — anchored to the layout viewport — visually displace.

## Approach (Solution E — sidestep root cause)

Instead of preventing the layout viewport from expanding, accept it and re-anchor the fixed elements to the *visual* viewport. The Visual Viewport API (`window.visualViewport`) exposes `offsetLeft`, `offsetTop`, and `resize`/`scroll` events that describe the visual viewport relative to the layout viewport. Listen for those events and apply a compensating `transform: translate()` to `.taskbar` and the Clippy widget root so they visually pin to the visual viewport regardless of layout viewport size.

This is the MDN-documented pattern for simulating `position: device-fixed`.

## Files

- `src/components/taskbar.rs` and `src/components/clippy.rs` (or a shared hook if cleaner) — add a `use_effect` that:
  1. Attaches `resize` and `scroll` listeners on `window.visualViewport`
  2. On each event, reads `visualViewport.offsetLeft` / `offsetTop` / `scale` and writes them to CSS custom properties (`--vv-offset-x`, `--vv-offset-y`) on the taskbar / Clippy root element.
  3. Drops the listeners on unmount.
- `styles/main.css` — add `transform: translate(var(--vv-offset-x, 0px), var(--vv-offset-y, 0px))` to the taskbar and Clippy rules.

## Todo

- [ ] Add `VisualViewport` feature to `web-sys` in `Cargo.toml`
- [ ] Implement Visual Viewport listener + CSS custom property writes in taskbar / Clippy components
- [ ] Add `transform: translate(...)` to `.taskbar` and Clippy root in `styles/main.css`
- [ ] Drop listeners on component unmount
- [ ] `cargo check --target wasm32-unknown-unknown` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown` passes with no new warnings

## Trade-offs

- One-frame flicker risk during rapid drag (transform fires after layout shift).
- Affects only the visual pinning — Webamp's drag still grows the body, which may cause unrelated secondary effects (e.g., horizontal scroll indicators in some browsers).
- Use as fallback if both `overflow: clip` (project-os-lzqc) and `renderInto()` (project-os-ym2f) don't pan out.

## Reasons for Scrapping

Deployed `0f01c04` and reverted via `4d4bb0a`. Two regressions observed on mobile:

1. Page spacing was disrupted (taskbar/Clippy transforms composited incorrectly — likely the `transform: translate(var(--vv-offset-x), var(--vv-offset-y))` clobbered or conflicted with existing transform context on the bottom-fixed taskbar + media-query overrides).
2. The Webamp window didn't render at all — the Visual Viewport hook may have interfered with the Clippy/taskbar mount order, OR the CSS transform moved elements off-screen, OR the `VisualViewport` listener triggered an early error that cascaded.

Not pursuing further; approach is more fragile than expected given the interplay with the existing fixed-position Win95 chrome and the mobile media-query layout.

All three candidate fixes in this batch failed:
- project-os-lzqc (overflow: clip): completed, no effect on the bug.
- project-os-ym2f (Webamp renderInto): scrapped, method not in public API.
- project-os-lhw3 (Visual Viewport): scrapped, caused regressions.

Need a fresh investigation round.
