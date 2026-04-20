use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Applies compensating CSS custom properties `--vv-offset-x` / `--vv-offset-y`
/// to the element referenced by `node_ref` so that a
/// `transform: translate(var(--vv-offset-x,0px), var(--vv-offset-y,0px))`
/// in CSS keeps `position:fixed` elements anchored to the *visual* viewport
/// even when the layout viewport is wider than the screen (e.g. due to Webamp
/// drag on mobile).
///
/// If `window.visualViewport` is not available (old browsers) this is a no-op
/// and the layout-viewport anchor remains unchanged.
#[hook]
pub fn use_visual_viewport_offset(node_ref: NodeRef) {
    use_effect_with(node_ref, |node_ref| {
        let node_ref = node_ref.clone();

        // Obtain window.visualViewport — bail silently if unavailable.
        let Some(vv) = web_sys::window().and_then(|w| w.visual_viewport()) else {
            return Box::new(|| ()) as Box<dyn FnOnce()>;
        };

        /// Write the current offsets to the CSS custom properties on the element.
        fn apply_offsets(node_ref: &NodeRef, vv: &web_sys::VisualViewport) {
            if let Some(el) = node_ref.cast::<web_sys::HtmlElement>() {
                let style = el.style();
                let _ = style.set_property(
                    "--vv-offset-x",
                    &format!("{}px", vv.offset_left()),
                );
                let _ = style.set_property(
                    "--vv-offset-y",
                    &format!("{}px", vv.offset_top()),
                );
            }
        }

        // Fire once on mount so the initial offset is correct immediately.
        apply_offsets(&node_ref, &vv);

        // VisualViewport implements EventTarget; cast it for gloo_events.
        let vv_et = vv.unchecked_ref::<web_sys::EventTarget>();

        let node_ref_resize = node_ref.clone();
        let vv_resize = vv.clone();
        let resize_listener = EventListener::new(vv_et, "resize", move |_| {
            apply_offsets(&node_ref_resize, &vv_resize);
        });

        let node_ref_scroll = node_ref.clone();
        let vv_scroll = vv.clone();
        let scroll_listener = EventListener::new(vv_et, "scroll", move |_| {
            apply_offsets(&node_ref_scroll, &vv_scroll);
        });

        // Listeners are dropped (and therefore unregistered) when the
        // cleanup closure runs on component unmount or dep change.
        Box::new(move || {
            drop(resize_listener);
            drop(scroll_listener);
        }) as Box<dyn FnOnce()>
    });
}
