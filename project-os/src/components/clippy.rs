use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use gloo_events::EventListener;

fn is_url(s: &str) -> bool {
    s.starts_with("http") || s.starts_with("data:") || s.contains('/')
}
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use crate::config::{ClippyConfig, Quote};

#[derive(Properties, PartialEq)]
pub struct ClippyProps {
    pub quotes: Vec<Quote>,
    pub clippy_config: ClippyConfig,
}

#[function_component(Clippy)]
pub fn clippy(props: &ClippyProps) -> Html {
    let quote_idx = use_state(|| 0usize);
    let modal_open = use_state(|| true);
    let bubble_visible = use_state(|| true);
    // Holds the pending 60s restore timer so we can cancel it on early re-show
    let hide_timer: UseStateHandle<Option<Timeout>> = use_state(|| None);
    // None = CSS bottom/right positioning; Some = dragged to (left, top)
    let pos: UseStateHandle<Option<(i32, i32)>> = use_state(|| None);
    let _move_listener: UseStateHandle<Option<EventListener>> = use_state(|| None);
    let _up_listener: UseStateHandle<Option<EventListener>> = use_state(|| None);
    // Stable ref to the widget div — avoids currentTarget issues in Yew's event system
    let widget_ref = use_node_ref();
    // Shared mutable flag: did the user drag since last mousedown?
    // use_mut_ref gives Rc<RefCell<T>> — all clones share the same cell, no stale-value issue
    let has_dragged = use_mut_ref(|| false);
    // Document-level click listener for "click outside to dismiss". Stored in a ref so
    // attaching/detaching it doesn't trigger an extra render cycle.
    let click_listener = use_mut_ref(|| None::<EventListener>);

    let current_quote = props.quotes.get(*quote_idx)
        .map(|q| q.text.clone())
        .unwrap_or_default();

    // Register or unregister the click-outside listener based on bubble + modal state.
    // The listener is active only when the bubble is visible and no modal is open.
    use_effect_with((*bubble_visible, *modal_open), {
        let click_listener = click_listener.clone();
        let widget_ref = widget_ref.clone();
        let bubble_visible = bubble_visible.clone();
        let hide_timer = hide_timer.clone();
        move |(is_visible, modal_is_open): &(bool, bool)| {
            if *is_visible && !*modal_is_open {
                let bv = bubble_visible.clone();
                let ht = hide_timer.clone();
                let wr = widget_ref.clone();
                let doc = web_sys::window().unwrap().document().unwrap();
                let listener = EventListener::new(&doc, "click", move |e| {
                    let target = e.dyn_ref::<MouseEvent>()
                        .and_then(|me| me.target())
                        .and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                    let inside = target.zip(wr.cast::<web_sys::Node>())
                        .map(|(t, w)| w.contains(Some(&t)))
                        .unwrap_or(false);
                    if !inside {
                        ht.set(None); // cancel any existing restore timer
                        bv.set(false);
                        let bv2 = bv.clone();
                        let timer = Timeout::new(60_000, move || bv2.set(true));
                        ht.set(Some(timer));
                    }
                });
                *click_listener.borrow_mut() = Some(listener);
            } else {
                *click_listener.borrow_mut() = None; // drops → JS removeEventListener
            }
            let cl = click_listener.clone();
            move || { *cl.borrow_mut() = None; } // cleanup when deps change
        }
    });

    let onmousedown = {
        let pos = pos.clone();
        let move_listener = _move_listener.clone();
        let up_listener = _up_listener.clone();
        let widget_ref = widget_ref.clone();
        let has_dragged = has_dragged.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // Reset drag flag on every new press
            *has_dragged.borrow_mut() = false;

            // Get the widget's current screen position via node ref (reliable, no currentTarget issues)
            let start_pos = match *pos {
                Some(p) => p,
                None => widget_ref
                    .cast::<web_sys::Element>()
                    .map(|el| {
                        let r = el.get_bounding_client_rect();
                        (r.left() as i32, r.top() as i32)
                    })
                    .unwrap_or((900, 450)),
            };

            let offset_x = e.client_x() - start_pos.0;
            let offset_y = e.client_y() - start_pos.1;

            let pos_mv = pos.clone();
            let move_ls_up = move_listener.clone();
            let has_dragged_mv = has_dragged.clone();

            let document = web_sys::window().unwrap().document().unwrap();

            let move_cb = EventListener::new(&document, "mousemove", move |e| {
                *has_dragged_mv.borrow_mut() = true;
                let e = e.dyn_ref::<MouseEvent>().unwrap();
                pos_mv.set(Some((e.client_x() - offset_x, e.client_y() - offset_y)));
            });
            let up_cb = EventListener::new(&document, "mouseup", move |_| {
                move_ls_up.set(None);
            });

            move_listener.set(Some(move_cb));
            up_listener.set(Some(up_cb));
        })
    };

    // Cycle to the next quote when the speech bubble is clicked
    let cycle_quote = {
        let qi = quote_idx.clone();
        let total = props.quotes.len();
        Callback::from(move |_: MouseEvent| {
            if total > 0 {
                qi.set((*qi + 1) % total);
            }
        })
    };

    // Icon click: if bubble is hidden, restore it and cancel any pending timer.
    // Otherwise open the info modal (existing behaviour).
    let open_modal = {
        let mo = modal_open.clone();
        let has_dragged = has_dragged.clone();
        let bubble_visible = bubble_visible.clone();
        let hide_timer = hide_timer.clone();
        Callback::from(move |_: MouseEvent| {
            if !*has_dragged.borrow() {
                if !*bubble_visible {
                    hide_timer.set(None); // drops Timeout → cancels JS timer
                    bubble_visible.set(true);
                } else {
                    mo.set(true);
                }
            }
        })
    };

    // Close the modal; the document click listener (re-registered on next render)
    // handles any subsequent "click outside" dismiss behaviour.
    let close_modal = {
        let mo = modal_open.clone();
        Callback::from(move |_: MouseEvent| mo.set(false))
    };

    let style = match *pos {
        Some((x, y)) => format!(
            "position:fixed; left:{}px; top:{}px; right:auto; bottom:auto; z-index:10;",
            x, y
        ),
        None => String::new(),
    };

    html! {
        <>
            <div class="clippy-widget" ref={widget_ref} style={style}>
                if *bubble_visible && !current_quote.is_empty() {
                    <div class="clippy-bubble" onclick={cycle_quote} title="Click to change quote">
                        <p>{ &current_quote }</p>
                    </div>
                }
                <div class="clippy-icon" onmousedown={onmousedown} onclick={open_modal} title="Drag to move · click for info">
                    if is_url(&props.clippy_config.icon) {
                        <img src={props.clippy_config.icon.clone()} alt="Clippy" />
                    } else {
                        { &props.clippy_config.icon }
                    }
                </div>
            </div>
            if *modal_open {
                <div class="clippy-modal-backdrop">
                    <div class="window clippy-modal">
                        <div class="title-bar">
                            <div class="title-bar-text">{ &props.clippy_config.modal_title }</div>
                        </div>
                        <div class="window-body" style="padding: 16px;">
                            <div style="display:flex;align-items:center;justify-content:center;gap:12px;margin-bottom:16px;">
                                <img src={props.clippy_config.logo.clone()} style="width:48px;height:48px;image-rendering:pixelated;" alt="logo" />
                                <span style="font-size:20px;font-weight:bold;">{ &props.clippy_config.brand_title }</span>
                            </div>
                            { for props.clippy_config.modal_paragraphs.iter().map(|p| html! {
                                <p>{ Html::from_html_unchecked(AttrValue::from(p.clone())) }</p>
                            }) }
                            <div style="text-align: right; margin-top: 12px;">
                                <button onclick={close_modal}>{ &props.clippy_config.modal_ok_label }</button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}
