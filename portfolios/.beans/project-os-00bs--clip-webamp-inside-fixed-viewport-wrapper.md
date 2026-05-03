---
# project-os-00bs
title: Clip Webamp inside fixed-viewport wrapper
status: todo
type: bug
priority: high
created_at: 2026-04-18T04:56:40Z
updated_at: 2026-04-18T04:56:40Z
---

## Problem

On mobile (Chrome Android), dragging the Webamp window horizontally pushes the Taskbar and Clippy widget progressively down and off the viewport. Prior fix in project-os-040x (`touch-action: none`, non-passive touchmove listener, `html,body { max-width: 100vw }`) stopped the page-pan symptom but did not prevent Webamp's absolutely-positioned `.window` chrome from painting past the body's right edge when dragged. When painted content exceeds the layout viewport width, mobile Chrome re-resolves the layout viewport — and fixed elements anchored to `bottom: 0` / `bottom: 10vh` shift downward visually relative to the visual viewport.

## Approach (Solution A)

Wrap `#webamp-mount` in a `position: fixed; inset: 0; overflow: hidden` container. This establishes an independent painting/clipping context pinned to the viewport — Webamp can move its chrome wherever it wants, but anything outside the wrapper is clipped and cannot influence document geometry.

## Files

- `styles/main.css` — add `.webamp-stage` rules near the existing `#webamp-mount` block:
  ```css
  .webamp-stage {
      position: fixed;
      inset: 0;
      overflow: hidden;
      pointer-events: none;
      z-index: 500;
  }
  .webamp-stage #webamp-mount {
      pointer-events: auto;
      position: absolute;
      inset: 0;
  }
  ```
- `src/components/webamp.rs:270-272` — wrap mount div:
  ```rust
  html! {
      <div class="webamp-stage">
          <div ref={mount_ref} id="webamp-mount"></div>
      </div>
  }
  ```

## Todo

- [ ] Add `.webamp-stage` CSS rules
- [ ] Wrap `#webamp-mount` in `.webamp-stage` in webamp.rs
- [ ] `cargo check --target wasm32-unknown-unknown` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown` passes with no new warnings
- [ ] Manual test: open Webamp, drag horizontally on mobile emulation — taskbar/Clippy stay put
- [ ] Verify no desktop regression

## Trade-offs

If user drags Webamp fully past a viewport edge, the player visibly clips and the title bar may become unreachable. This is acceptable per user direction; the alternative clamp approach is being pursued in parallel in a sibling ticket.
