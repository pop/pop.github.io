use yew::prelude::*;
use crate::config::load_config;
use crate::components::desktop::Desktop;

#[function_component(App)]
pub fn app() -> Html {
    let config = load_config();
    html! {
        <Desktop games={config.games} />
    }
}
