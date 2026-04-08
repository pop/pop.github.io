use yew::prelude::*;
use crate::config::Game;

#[derive(Properties, PartialEq)]
pub struct GameWindowProps {
    pub game: Game,
}

#[function_component(GameWindow)]
pub fn game_window(props: &GameWindowProps) -> Html {
    let game = &props.game;

    let launch_label = if game.launch_type == "wasm" { "Launch" } else { "Download" };
    let launch_url = game.launch_url.clone();

    let on_launch = Callback::from(move |_: MouseEvent| {
        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url_and_target(&launch_url, "_blank");
        }
    });

    let demo_html = match &game.demo {
        Some(url) if url.ends_with(".mp4") || url.ends_with(".webm") => html! {
            <video class="game-demo" src={url.clone()} autoplay=true loop=true muted=true />
        },
        Some(url) => html! {
            <img class="game-demo" src={url.clone()} alt="demo" />
        },
        None => html! {
            <div class="game-demo game-demo-placeholder"></div>
        },
    };

    html! {
        <div class="game-window-content">
            <h2>{ &game.title }</h2>
            if !game.contributors.is_empty() {
                <p class="contributors">{ format!("By: {}", game.contributors.join(", ")) }</p>
            }
            <div class="tech-stack">
                { for game.tech.iter().map(|t| {
                    let icon_src = if t.icon.is_empty() {
                        "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16'><rect width='16' height='16' fill='%23808080'/></svg>".to_string()
                    } else {
                        t.icon.clone()
                    };
                    html! {
                        <span class="tech-item" key={t.name.clone()}>
                            <img src={icon_src} alt={t.name.clone()} />
                            { &t.name }
                        </span>
                    }
                })}
            </div>
            <p class="description">{ &game.description }</p>
            { demo_html }
            <div class="launch-row">
                <button class="button" onclick={on_launch}>{ launch_label }</button>
            </div>
        </div>
    }
}
