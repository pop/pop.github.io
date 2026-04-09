use yew::prelude::*;
use crate::config::StartMenu;

#[derive(Properties, PartialEq)]
pub struct StartMenuProps {
    pub config: StartMenu,
    pub visible: bool,
    pub on_close: Callback<()>,
}

#[function_component(StartMenuComp)]
pub fn start_menu(props: &StartMenuProps) -> Html {
    if !props.visible {
        return html! {};
    }

    let open = |url: String| {
        Callback::from(move |_: MouseEvent| {
            if let Some(w) = web_sys::window() {
                let _ = w.open_with_url_and_target(&url, "_blank");
            }
        })
    };

    let shutdown_message = props.config.shutdown_message.clone();
    let on_shutdown = Callback::from(move |_: MouseEvent| {
        if let Some(w) = web_sys::window() {
            let _ = w.alert_with_message(&shutdown_message);
        }
    });

    let on_close = props.on_close.clone();
    let backdrop_close = Callback::from(move |_: MouseEvent| on_close.emit(()));

    html! {
        <>
            // Backdrop to catch outside clicks
            <div class="start-menu-backdrop" onclick={backdrop_close}></div>
            <div class="start-menu window">
                <ul class="start-menu-list">
                    <li>
                        <button class="start-menu-item" onclick={open(props.config.about_url.clone())}>
                            { &props.config.about_label }
                        </button>
                    </li>
                    <li>
                        <button class="start-menu-item" onclick={open(props.config.github_url.clone())}>
                            { &props.config.github_label }
                        </button>
                    </li>
                    <li>
                        <button class="start-menu-item" onclick={open(props.config.itchio_url.clone())}>
                            { &props.config.itchio_label }
                        </button>
                    </li>
                    <li><hr /></li>
                    <li>
                        <button class="start-menu-item start-menu-shutdown" onclick={on_shutdown}>
                            { &props.config.shutdown_label }
                        </button>
                    </li>
                </ul>
            </div>
        </>
    }
}
