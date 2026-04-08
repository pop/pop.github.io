use yew::prelude::*;
use crate::config::Game;

#[derive(Properties, PartialEq)]
pub struct DesktopProps {
    pub games: Vec<Game>,
}

#[function_component(Desktop)]
pub fn desktop(props: &DesktopProps) -> Html {
    html! {
        <div id="desktop">
            <div class="icon-grid">
                { for props.games.iter().map(|g| html! {
                    <span key={g.id.clone()} class="game-icon-placeholder">{ &g.title }</span>
                })}
            </div>
        </div>
    }
}
