use yew::prelude::*;
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
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

    // Rotate quotes every 5s
    {
        let qi = quote_idx.clone();
        let total = props.quotes.len();
        use_effect_with(total, move |_| -> Box<dyn FnOnce()> {
            if total == 0 {
                return Box::new(move || {});
            }
            let interval = Interval::new(5000, move || {
                qi.set((*qi + 1) % total);
            });
            Box::new(move || drop(interval))
        });
    }

    let current_quote = props.quotes.get(*quote_idx)
        .map(|q| q.text.clone())
        .unwrap_or_default();

    let onmousedown = {
        let pos = pos.clone();
        let move_listener = _move_listener.clone();
        let up_listener = _up_listener.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            // Get current top-left of the widget from DOM or stored state
            let start_pos = match *pos {
                Some(p) => p,
                None => e.current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
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

            let document = web_sys::window().unwrap().document().unwrap();

            let move_cb = EventListener::new(&document, "mousemove", move |e| {
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

    let open_modal = {
        let mo = modal_open.clone();
        Callback::from(move |_: MouseEvent| mo.set(true))
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
            <div class="clippy-widget" style={style} onmousedown={onmousedown}>
                if !current_quote.is_empty() {
                    <div class="clippy-bubble">
                        <p>{ &current_quote }</p>
                    </div>
                }
                <div class="clippy-icon" onclick={open_modal} title="Click for info">
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
