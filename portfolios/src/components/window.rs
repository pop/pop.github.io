use yew::prelude::*;
use gloo_events::{EventListener, EventListenerOptions};
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, TouchEvent};

#[derive(Properties, PartialEq)]
pub struct WindowProps {
    pub title: String,
    pub z_index: u32,
    pub pos: (i32, i32),
    pub on_close: Callback<()>,
    pub on_focus: Callback<()>,
    pub on_move: Callback<(i32, i32)>,
    pub children: Children,
}

#[function_component(Window)]
pub fn window(props: &WindowProps) -> Html {
    let drag_offset: UseStateHandle<Option<(i32, i32)>> = use_state(|| None);

    // Keep listeners alive for the duration of the drag
    let _move_listener: UseStateHandle<Option<EventListener>> = use_state(|| None);
    let _up_listener: UseStateHandle<Option<EventListener>> = use_state(|| None);
    // Non-passive touchstart listener on the title bar (kept alive between renders)
    let _touch_start_listener: UseStateHandle<Option<EventListener>> = use_state(|| None);

    // Ref to the title-bar element so we can attach a non-passive touchstart
    let title_bar_ref = use_node_ref();

    // Per-window jitter — initialized once on mount, used by mobile CSS transform
    let jitter_x = use_state(|| ((js_sys::Math::random() * 20.0) as i32) - 10);
    let jitter_y = use_state(|| (js_sys::Math::random() * 40.0) as i32);

    let style = format!(
        "position:absolute; left:{}px; top:{}px; z-index:{}; width:480px; min-height:300px; --jitter-x:{}px; --jitter-y:{}px;",
        props.pos.0, props.pos.1, props.z_index, *jitter_x, *jitter_y
    );

    // --- mouse drag on title bar ---
    let onmousedown = {
        let on_focus = props.on_focus.clone();
        let on_move = props.on_move.clone();
        let pos = props.pos;
        let drag_offset = drag_offset.clone();
        let move_listener_state = _move_listener.clone();
        let up_listener_state = _up_listener.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_focus.emit(());

            let offset_x = e.client_x() - pos.0;
            let offset_y = e.client_y() - pos.1;
            drag_offset.set(Some((offset_x, offset_y)));

            let on_move_mv = on_move.clone();
            let drag_offset_up = drag_offset.clone();
            let move_listener_state_up = move_listener_state.clone();
            let offset = (offset_x, offset_y);

            let document = web_sys::window().unwrap().document().unwrap();

            let move_cb = EventListener::new(&document, "mousemove", move |e| {
                let e = e.dyn_ref::<MouseEvent>().unwrap();
                on_move_mv.emit((e.client_x() - offset.0, e.client_y() - offset.1));
            });
            let up_cb = EventListener::new(&document, "mouseup", move |_| {
                drag_offset_up.set(None);
                move_listener_state_up.set(None);
            });

            move_listener_state.set(Some(move_cb));
            up_listener_state.set(Some(up_cb));
        })
    };

    // --- non-passive touch drag on title bar ---
    // Attach via gloo_events::EventListener::new_with_options so we can call
    // preventDefault() — Yew's ontouchstart prop registers a passive listener
    // which prevents preventDefault(), causing the browser to treat the gesture
    // as a page-pan and swallow subsequent touchmove events.
    {
        let title_bar_ref = title_bar_ref.clone();
        let on_focus = props.on_focus.clone();
        let on_move = props.on_move.clone();
        let pos = props.pos;
        let drag_offset = drag_offset.clone();
        let move_listener_state = _move_listener.clone();
        let up_listener_state = _up_listener.clone();
        let touch_start_listener = _touch_start_listener.clone();

        use_effect_with(
            (title_bar_ref.clone(), pos),
            move |(title_bar_ref, pos)| {
                let pos = *pos;
                let Some(el) = title_bar_ref.cast::<web_sys::HtmlElement>() else {
                    return;
                };
                let el_et: &web_sys::EventTarget = el.as_ref();

                let opts = EventListenerOptions::enable_prevent_default();
                let listener = EventListener::new_with_options(el_et, "touchstart", opts, move |event| {
                    let e = event.dyn_ref::<TouchEvent>().unwrap();
                    e.prevent_default();
                    on_focus.emit(());

                    if let Some(touch) = e.touches().get(0) {
                        let offset_x = touch.client_x() - pos.0;
                        let offset_y = touch.client_y() - pos.1;
                        drag_offset.set(Some((offset_x, offset_y)));

                        let on_move_mv = on_move.clone();
                        let drag_offset_end = drag_offset.clone();
                        let move_listener_state_end = move_listener_state.clone();
                        let offset = (offset_x, offset_y);

                        let document = web_sys::window().unwrap().document().unwrap();

                        let move_cb = EventListener::new(&document, "touchmove", move |e| {
                            let e = e.dyn_ref::<TouchEvent>().unwrap();
                            if let Some(t) = e.touches().get(0) {
                                on_move_mv.emit((t.client_x() - offset.0, t.client_y() - offset.1));
                            }
                        });
                        let up_cb = EventListener::new(&document, "touchend", move |_| {
                            drag_offset_end.set(None);
                            move_listener_state_end.set(None);
                        });

                        move_listener_state.set(Some(move_cb));
                        up_listener_state.set(Some(up_cb));
                    }
                });

                touch_start_listener.set(Some(listener));
            },
        );
    }

    // clicking anywhere on the window brings it to focus
    let onclick_window = {
        let on_focus = props.on_focus.clone();
        Callback::from(move |_: MouseEvent| {
            on_focus.emit(());
        })
    };

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_close.emit(());
        })
    };

    html! {
        <div class="window" style={style} onclick={onclick_window}>
            <div class="title-bar" ref={title_bar_ref} onmousedown={onmousedown}>
                <div class="title-bar-text">{ &props.title }</div>
                <div class="title-bar-controls">
                    <button aria-label="Close" onclick={on_close_click}></button>
                </div>
            </div>
            <div class="window-body" style="height: calc(100% - 32px); overflow: auto;">
                { for props.children.iter() }
            </div>
        </div>
    }
}
