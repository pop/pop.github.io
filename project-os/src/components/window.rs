use yew::prelude::*;
use gloo_events::EventListener;
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

    let style = format!(
        "position:absolute; left:{}px; top:{}px; z-index:{}; width:480px; min-height:300px;",
        props.pos.0, props.pos.1, props.z_index
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

    // --- touch drag on title bar ---
    let ontouchstart = {
        let on_focus = props.on_focus.clone();
        let on_move = props.on_move.clone();
        let pos = props.pos;
        let drag_offset = drag_offset.clone();
        let move_listener_state = _move_listener.clone();
        let up_listener_state = _up_listener.clone();

        Callback::from(move |e: TouchEvent| {
            e.prevent_default();
            on_focus.emit(());

            if let Some(touch) = e.touches().get(0) {
                let offset_x = touch.client_x() - pos.0;
                let offset_y = touch.client_y() - pos.1;
                drag_offset.set(Some((offset_x, offset_y)));

                let on_move_mv = on_move.clone();
                let drag_offset_mv = drag_offset.clone();
                let drag_offset_end = drag_offset.clone();
                let move_listener_state_end = move_listener_state.clone();
                let offset = (offset_x, offset_y);

                let document = web_sys::window().unwrap().document().unwrap();

                let move_cb = EventListener::new(&document, "touchmove", move |e| {
                    if (*drag_offset_mv).is_none() { return; }
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
        })
    };

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
            <div class="title-bar" onmousedown={onmousedown} ontouchstart={ontouchstart}>
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
