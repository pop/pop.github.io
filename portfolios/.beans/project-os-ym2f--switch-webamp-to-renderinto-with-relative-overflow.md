---
# project-os-ym2f
title: Switch Webamp to renderInto() with relative overflow-hidden container
status: todo
type: bug
priority: high
created_at: 2026-04-19T23:14:06Z
updated_at: 2026-04-19T23:14:06Z
---

## Problem

Same root cause as project-os-lzqc: Webamp attaches its chrome to `document.body` and clamps drag to `body.scrollWidth`. Dragging grows the body width → mobile Chrome re-resolves the layout viewport → fixed taskbar and Clippy visually displace.

## Approach (Solution D — most promising fix)

Webamp exposes a `renderInto(target)` method (public method on the instance, enforced with a runtime guard: the target must be non-static positioned). Unlike `renderWhenReady`, `renderInto` flips Webamp's internal `parentDomNode` to the passed element, and the drag clamp resolves to `getElementSize(target)` instead of the body. By sizing the target to `100vw × 100vh` with `overflow: hidden`, we:

1. Keep Webamp's coordinate math correct (target stays `position: relative`, normal offset chain — this is why Solution A with `position: fixed` broke interactivity).
2. Prevent the body from ever growing, because any paint outside the container is clipped at the viewport.

## Files

- `src/components/webamp.rs:11-24` — add a `renderInto` binding alongside `renderWhenReady`:
  ```rust
  #[wasm_bindgen(method, js_name = renderInto)]
  fn render_into(this: &JsWebamp, target: &web_sys::Element) -> js_sys::Promise;
  ```
- `src/components/webamp.rs:184` — swap `wa.render_when_ready(&target)` for `wa.render_into(&target)`.
- `styles/main.css` — update the `#webamp-mount` rule:
  ```css
  #webamp-mount {
      position: relative;
      width: 100vw;
      height: 100vh;
      overflow: hidden;
      /* keep existing touch-action: none */
  }
  ```

## Todo

- [ ] Add `renderInto` binding in the `extern "C"` block
- [ ] Swap `render_when_ready(&target)` → `render_into(&target)` at the call site
- [ ] Update `#webamp-mount` CSS: `position: relative; width: 100vw; height: 100vh; overflow: hidden`
- [ ] Keep `touch-action: none` on `#webamp-mount` from prior fix
- [ ] `cargo check --target wasm32-unknown-unknown` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown` passes with no new warnings
- [ ] Verify no regression on desktop (windows draggable, not cut off)

## Trade-offs

- Webamp windows will now clamp to the viewport edges on mobile AND desktop. On desktop, prior behavior allowed dragging windows anywhere in `document.body`; new behavior clamps to the viewport. This is arguably a UX improvement (windows can't get "lost"), but callers should be aware.
- `renderInto` is not in the Webamp npm README but is exported as a public method on every Webamp instance (see `webampLazy.tsx` in the captbaritone/webamp repo).
