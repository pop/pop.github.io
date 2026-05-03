use yew::prelude::*;
use crate::config::Game;

fn is_url(s: &str) -> bool {
    s.starts_with("http") || s.starts_with("data:") || s.contains('/')
}

#[derive(Properties, PartialEq)]
pub struct GameWindowProps {
    pub game: Game,
}

#[function_component(GameWindow)]
pub fn game_window(props: &GameWindowProps) -> Html {
    let game = &props.game;

    let launch_label = game.launch_label.clone();
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
            <div class="game-window-left">
                <h2>{ &game.title }</h2>
                if !game.contributors.is_empty() {
                    <p class="contributors">{ format!("By: {}", game.contributors.join(", ")) }</p>
                }
                <div class="tech-stack">
                    { for game.tech.iter().map(|t| {
                        let tech_icon = if is_url(&t.icon) {
                            html! { <img src={t.icon.clone()} alt={t.name.clone()} /> }
                        } else if !t.icon.is_empty() {
                            html! { <span class="tech-icon-emoji">{ &t.icon }</span> }
                        } else {
                            html! { <img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16'><rect width='16' height='16' fill='%23808080'/></svg>" alt={t.name.clone()} /> }
                        };
                        let tech_name = match &t.url {
                            Some(url) => html! { <a href={url.clone()} target="_blank">{ &t.name }</a> },
                            None => html! { { &t.name } },
                        };
                        html! {
                            <span class="tech-item" key={t.name.clone()}>
                                { tech_icon }
                                { tech_name }
                            </span>
                        }
                    })}
                </div>
                <p class="description">{ Html::from_html_unchecked(AttrValue::from(game.description.clone())) }</p>
                <div class="launch-row">
                    <button class="button" onclick={on_launch}>{ &launch_label }</button>
                </div>
            </div>
            { demo_html }
        </div>
    }
}
