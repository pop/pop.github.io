use yew::prelude::*;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use crate::config::Quote;

#[derive(Properties, PartialEq)]
pub struct ClippyProps {
    pub quotes: Vec<Quote>,
}

#[function_component(Clippy)]
pub fn clippy(props: &ClippyProps) -> Html {
    let quote_idx = use_state(|| 0usize);
    let modal_open = use_state(|| false);
    // None = CSS bottom/right positioning; Some = dragged to (left, top)
    let pos: UseStateHandle<Option<(i32, i32)>> = use_state(|| None);
    let _move_listener: UseStateHandle<Option<EventListener>> = use_state(|| None);
    let _up_listener: UseStateHandle<Option<EventListener>> = use_state(|| None);
    // Stable ref to the widget div — avoids currentTarget issues in Yew's event system
    let widget_ref = use_node_ref();
    // Shared mutable flag: did the user drag since last mousedown?
    // use_mut_ref gives Rc<RefCell<T>> — all clones share the same cell, no stale-value issue
    let has_dragged = use_mut_ref(|| false);

    let current_quote = props.quotes.get(*quote_idx)
        .map(|q| q.text.clone())
        .unwrap_or_default();

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

    // Only open modal if the user clicked the icon without dragging
    let open_modal = {
        let mo = modal_open.clone();
        let has_dragged = has_dragged.clone();
        Callback::from(move |_: MouseEvent| {
            if !*has_dragged.borrow() {
                mo.set(true);
            }
        })
    };
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
                if !current_quote.is_empty() {
                    <div class="clippy-bubble" onclick={cycle_quote} title="Click to change quote">
                        <p>{ &current_quote }</p>
                    </div>
                }
                <div class="clippy-icon" onmousedown={onmousedown} onclick={open_modal} title="Drag to move · click for info">
                    { "📎" }
                </div>
            </div>
            if *modal_open {
                <div class="clippy-modal-backdrop">
                    <div class="window clippy-modal">
                        <div class="title-bar">
                            <div class="title-bar-text">{ "About this Portfolio" }</div>
                        </div>
                        <div class="window-body" style="padding: 16px;">
                            <p>{ "This portfolio was built with assistance from Claude AI (Anthropic)." }</p>
                            <p>{ "The games and creative work are by the portfolio author." }</p>
                            <p>{ "Claude helped with Rust/WASM code and UI implementation." }</p>
                            <div style="text-align: right; margin-top: 12px;">
                                <button onclick={close_modal}>{ "OK" }</button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}
