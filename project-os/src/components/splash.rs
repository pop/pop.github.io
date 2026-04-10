use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::config::SplashConfig;
use crate::components::window::Window;

#[derive(Properties, PartialEq)]
pub struct SplashProps {
    pub config: SplashConfig,
    pub on_dismiss: Callback<()>,
}

#[function_component(Splash)]
pub fn splash(props: &SplashProps) -> Html {
    let pos = use_state(|| (300i32, 150i32));
    let z_index = use_state(|| 500u32);
    let node_ref = use_node_ref();
    let click_listener = use_mut_ref(|| None::<EventListener>);

    use_effect_with((), {
        let on_dismiss = props.on_dismiss.clone();
        let node_ref = node_ref.clone();
        let click_listener = click_listener.clone();
        move |_| {
            let doc = web_sys::window().unwrap().document().unwrap();
            let nr = node_ref.clone();
            let cb = on_dismiss.clone();
            let listener = EventListener::new(&doc, "click", move |e| {
                let target = e.dyn_ref::<web_sys::MouseEvent>()
                    .and_then(|me| me.target())
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                let inside = target
                    .zip(nr.cast::<web_sys::Node>())
                    .map(|(t, w)| w.contains(Some(&t)))
                    .unwrap_or(false);
                if !inside {
                    cb.emit(());
                }
            });
            *click_listener.borrow_mut() = Some(listener);
            let cl = click_listener.clone();
            move || { *cl.borrow_mut() = None; }
        }
    });

    let on_move = {
        let pos = pos.clone();
        Callback::from(move |p: (i32, i32)| pos.set(p))
    };

    let on_focus = {
        let z_index = z_index.clone();
        Callback::from(move |_: ()| {
            let new_z = *z_index + 1;
            z_index.set(new_z);
        })
    };

    html! {
        <div ref={node_ref}>
            <Window
                title={props.config.title.clone()}
                z_index={*z_index}
                pos={*pos}
                on_close={props.on_dismiss.clone()}
                on_focus={on_focus}
                on_move={on_move}
            >
                <div style="padding:24px;display:flex;align-items:center;gap:16px;">
                    <img src={props.config.logo.clone()} style="width:64px;height:64px;image-rendering:pixelated;" alt="logo" />
                    <span style="font-size:24px;font-weight:bold;">{ &props.config.title }</span>
                </div>
            </Window>
        </div>
    }
}
