use yew::prelude::*;
use crate::config::load_config;
use crate::components::desktop::Desktop;

#[function_component(App)]
pub fn app() -> Html {
    let config = load_config();
    let on_open = Callback::from(|id: String| {
        web_sys::console::log_1(&format!("open window: {id}").into());
    });
    html! {
        <Desktop games={config.games} {on_open} />
    }
}
