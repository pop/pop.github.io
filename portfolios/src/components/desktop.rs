use yew::prelude::*;
use crate::config::Game;
use super::game_icon::GameIcon;

#[derive(Properties, PartialEq)]
pub struct DesktopProps {
    pub projects: Vec<Game>,
    pub on_open: Callback<String>,
    pub background_color: String,
}

#[function_component(Desktop)]
pub fn desktop(props: &DesktopProps) -> Html {
    let style = format!("background-color: {};", props.background_color);
    html! {
        <div id="desktop" style={style}>
            <div class="icon-grid">
                { for props.projects.iter().map(|g| html! {
                    <GameIcon key={g.id.clone()} game={g.clone()} on_open={props.on_open.clone()} />
                })}
            </div>
        </div>
    }
}
