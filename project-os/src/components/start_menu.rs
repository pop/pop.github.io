use yew::prelude::*;
use crate::config::{StartMenu, StartMenuItem};

fn is_url(s: &str) -> bool {
    s.starts_with("http") || s.starts_with("data:") || s.contains('/')
}

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

    let on_close = props.on_close.clone();
    let backdrop_close = Callback::from(move |_: MouseEvent| on_close.emit(()));

    let items = props.config.items.iter().map(|item: &StartMenuItem| {
        let icon_html = if is_url(&item.icon) {
            html! { <img src={item.icon.clone()} alt="" class="start-menu-icon" /> }
        } else if !item.icon.is_empty() {
            html! { <span class="start-menu-icon-emoji">{ &item.icon }</span> }
        } else {
            html! {}
        };
        html! {
            <li>
                <button class="start-menu-item" onclick={open(item.url.clone())}>
                    { icon_html }
                    { &item.title }
                </button>
            </li>
        }
    }).collect::<Html>();

    html! {
        <>
            // Backdrop to catch outside clicks
            <div class="start-menu-backdrop" onclick={backdrop_close}></div>
            <div class="start-menu window">
                <ul class="start-menu-list">
                    { items }
                </ul>
            </div>
        </>
    }
}
