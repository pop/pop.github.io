---
# project-os-4umw
title: Clamp Webamp window positions to viewport via MutationObserver
status: todo
type: bug
priority: high
created_at: 2026-04-18T04:57:02Z
updated_at: 2026-04-18T04:57:02Z
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

- [ ] Add `MutationObserver` that clamps `style.left`/`style.top` on descendants
- [ ] Avoid reentrance loops (tolerance check before write-back)
- [ ] Disconnect observer on component unmount
- [ ] `cargo check --target wasm32-unknown-unknown` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown` passes with no new warnings
- [ ] Manual test: drag Webamp to viewport edges on mobile emulation — player stops at edge, taskbar/Clippy stay put
- [ ] Verify no desktop regression, no drag stutter
