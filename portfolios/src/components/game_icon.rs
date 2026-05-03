use yew::prelude::*;
use crate::config::Game;

#[derive(Properties, PartialEq)]
pub struct GameIconProps {
    pub game: Game,
    pub on_open: Callback<String>,
}

fn is_url(s: &str) -> bool {
    s.starts_with("http") || s.starts_with("data:") || s.contains('/')
}

#[function_component(GameIcon)]
pub fn game_icon(props: &GameIconProps) -> Html {
    let game_id = props.game.id.clone();
    let on_open = props.on_open.clone();
    let onclick = Callback::from(move |_| on_open.emit(game_id.clone()));

    let icon = if is_url(&props.game.icon) {
        html! { <img src={props.game.icon.clone()} alt={props.game.title.clone()} /> }
    } else if !props.game.icon.is_empty() {
        html! { <span class="game-icon-emoji">{ &props.game.icon }</span> }
    } else {
        html! { <img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='48' height='48'><rect width='48' height='48' fill='%23808080'/></svg>" alt={props.game.title.clone()} /> }
    };

    html! {
        <div class="game-icon" {onclick}>
            { icon }
            <span class="game-icon-label">{ &props.game.title }</span>
        </div>
    }
}
