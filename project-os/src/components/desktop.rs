use yew::prelude::*;
use crate::config::Game;
use super::game_icon::GameIcon;

#[derive(Properties, PartialEq)]
pub struct DesktopProps {
    pub games: Vec<Game>,
    pub on_open: Callback<String>,
}

#[function_component(Desktop)]
pub fn desktop(props: &DesktopProps) -> Html {
    html! {
        <div id="desktop">
            <div class="icon-grid">
                { for props.games.iter().map(|g| html! {
                    <GameIcon key={g.id.clone()} game={g.clone()} on_open={props.on_open.clone()} />
                })}
            </div>
        </div>
    }
}
