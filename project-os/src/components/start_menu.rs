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

    let on_shutdown = Callback::from(|_: MouseEvent| {
        if let Some(w) = web_sys::window() {
            let _ = w.alert_with_message("It is now safe to turn off your computer.");
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
                            { "\u{1F4C4} About" }
                        </button>
                    </li>
                    <li>
                        <button class="start-menu-item" onclick={open(props.config.github_url.clone())}>
                            { "\u{1F419} GitHub" }
                        </button>
                    </li>
                    <li>
                        <button class="start-menu-item" onclick={open(props.config.itchio_url.clone())}>
                            { "\u{1F3AE} itch.io" }
                        </button>
                    </li>
                    <li><hr /></li>
                    <li>
                        <button class="start-menu-item start-menu-shutdown" onclick={on_shutdown}>
                            { "\u{23FB} Shut Down..." }
                        </button>
                    </li>
                </ul>
            </div>
        </>
    }
}
