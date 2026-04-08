use yew::prelude::*;
use crate::config::Game;

#[derive(Properties, PartialEq)]
pub struct GameIconProps {
    pub game: Game,
    pub on_open: Callback<String>,
}

#[function_component(GameIcon)]
pub fn game_icon(props: &GameIconProps) -> Html {
    let game_id = props.game.id.clone();
    let on_open = props.on_open.clone();
    let onclick = Callback::from(move |_| on_open.emit(game_id.clone()));

    let icon_src = if props.game.icon.is_empty() {
        "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='48' height='48'><rect width='48' height='48' fill='%23808080'/></svg>".to_string()
    } else {
        props.game.icon.clone()
    };

    html! {
        <div class="game-icon" {onclick}>
            <img src={icon_src} alt={props.game.title.clone()} />
            <span class="game-icon-label">{ &props.game.title }</span>
        </div>
    }
}
