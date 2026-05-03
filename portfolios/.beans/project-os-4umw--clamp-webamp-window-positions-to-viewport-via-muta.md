---
# project-os-4umw
title: Clamp Webamp window positions to viewport via MutationObserver
status: scrapped
type: bug
priority: high
created_at: 2026-04-18T04:57:02Z
updated_at: 2026-04-23T20:35:26Z
---

## Problem

Same root cause as project-os-00bs: on mobile, Webamp's internal drag moves its absolutely-positioned `.window` chrome (via inline `style.left`/`style.top`) past the body's right edge. When painted content exceeds the layout viewport width, mobile Chrome re-resolves the layout viewport and pushes fixed-position Taskbar and Clippy out of the visual viewport.

Prior fix in project-os-040x did not solve this — `touch-action: none` and `html,body { max-width: 100vw }` stop page panning but do not constrain where Webamp positions its own chrome.

## Approach (Solution B)

Attach a `MutationObserver` to `#webamp-mount` that watches for `style` attribute mutations on its descendants. After each mutation, read the target's inline `left`/`top`, clamp to `[0, window.innerWidth - offsetWidth]` / `[0, window.innerHeight - offsetHeight]`, and write back if out of range. Guard against reentrance loops (the clamp write is itself a style mutation).

## Files

- `src/components/webamp.rs` — inside the `init` closure, after the non-passive `touchmove` listener is registered (around line 208), construct a `MutationObserver` via `web_sys`. Store the observer and its `Closure` in `Rc<RefCell<Option<_>>>` handles alongside the existing `touch_listener`/`pending_listener` refs so they're dropped in the effect cleanup.

## Implementation sketch

```rust
use web_sys::{MutationObserver, MutationObserverInit};
use wasm_bindgen::closure::Closure;

let target_el = target.clone();
let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |records: js_sys::Array| {
    let window = web_sys::window().unwrap();
    let vw = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
    let vh = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
    for i in 0..records.length() {
        let rec: web_sys::MutationRecord = records.get(i).unchecked_into();
        let Some(node) = rec.target() else { continue };
        let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() else { continue };
        let w = el.offset_width() as f64;
        let h = el.offset_height() as f64;
        let style = el.style();
        let left = style.get_property_value("left").ok()
            .and_then(|s| s.trim_end_matches("px").parse::<f64>().ok());
        let top = style.get_property_value("top").ok()
            .and_then(|s| s.trim_end_matches("px").parse::<f64>().ok());
        if let Some(l) = left {
            let clamped = l.clamp(0.0, (vw - w).max(0.0));
            if (clamped - l).abs() > 0.5 {
                let _ = style.set_property("left", &format!("{clamped}px"));
            }
        }
        if let Some(t) = top {
            let clamped = t.clamp(0.0, (vh - h).max(0.0));
            if (clamped - t).abs() > 0.5 {
                let _ = style.set_property("top", &format!("{clamped}px"));
            }
        }
    }
});
let observer = MutationObserver::new(callback.as_ref().unchecked_ref()).unwrap();
let init = MutationObserverInit::new();
init.set_subtree(true);
init.set_attributes(true);
let filter = js_sys::Array::new();
filter.push(&"style".into());
init.set_attribute_filter(&filter);
observer.observe_with_options(&target_el, &init).unwrap();
```

Store both `observer` and `callback` in RefCell handles; on cleanup call `observer.disconnect()` and drop.

## Todo

- [x] Add `MutationObserver` that clamps `style.left`/`style.top` on descendants
- [x] Avoid reentrance loops (tolerance check before write-back)
- [x] Disconnect observer on component unmount
- [x] `cargo check --target wasm32-unknown-unknown` passes
- [x] `cargo clippy --target wasm32-unknown-unknown` passes with no new warnings
- [ ] Manual test: drag Webamp to viewport edges on mobile emulation — player stops at edge, taskbar/Clippy stay put
- [ ] Verify no desktop regression, no drag stutter

## Update 2026-04-20

Firefox RDM repro confirmed the root cause empirically: during Webamp drag, `window.innerWidth` grows while `window.visualViewport.width` stays stable. That's Firefox's layout viewport auto-expanding to contain Webamp's overflowing chrome — the exact mechanism this clamp prevents. Taskbar/Clippy are anchored to the layout viewport, which is why they visually drift.

Implementing this ticket should resolve the Firefox RDM symptom documented in project-os-s0wm.

## Summary of Changes

Added a `MutationObserver` inside the `init` closure in `src/components/webamp.rs` that watches `style` attribute mutations on all descendants of `#webamp-mount`. On each mutation the callback reads the mutated element's inline `left`/`top`, clamps them to `[0, innerWidth - offsetWidth]` / `[0, innerHeight - offsetHeight]`, and writes back only when the delta exceeds 0.5 px (reentrance guard). The observer and its backing `Closure` are stored in `Rc<RefCell<Option<_>>>` handles and disconnected/dropped in the effect cleanup. Required adding `MutationObserver`, `MutationObserverInit`, `MutationRecord`, `Node`, and `CssStyleDeclaration` to the web-sys feature list in `Cargo.toml`. Introduced a `MutationClosureHandle` type alias to satisfy clippy's `type_complexity` lint. Committed as f5147d8 on branch `worktree-agent-af8921dc`.


## Reasons for Scrapping

Abandoned on 2026-04-23. The MutationObserver clamp on `#webamp-mount` subtree was shipped (commit `f5147d8` on branch `worktree-agent-af8921dc`) and deployed to Cloudflare Pages — the user verified the Firefox RDM taskbar/Clippy drift persisted. Root cause hypothesis: the clamp observes `#webamp-mount` but Webamp injects `#main-window` directly into `<body>`, so the observer never fires on the elements that actually overflow. Even if retargeted to `<body>`, the scrapped follow-up ticket project-os-l810 showed that neither clamp nor explicit position writes affect Webamp's rendered position.

This approach is scrapped. The Firefox RDM symptom in project-os-s0wm remains unresolved.
