use yew::prelude::*;
use gloo_timers::callback::Interval;
use crate::config::Game;
use crate::state::WindowState;

#[derive(Properties, PartialEq)]
pub struct TaskbarProps {
    pub windows: Vec<WindowState>,
    pub games: Vec<Game>,
    pub on_focus: Callback<String>,
    pub on_start_click: Callback<()>,
    pub start_menu_open: bool,
}

fn current_time() -> String {
    let date = js_sys::Date::new_0();
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    let (h, ampm) = if hours >= 12 {
        (if hours == 12 { 12 } else { hours - 12 }, "PM")
    } else {
        (if hours == 0 { 12 } else { hours }, "AM")
    };
    format!("{:02}:{:02} {}", h, minutes, ampm)
}

#[function_component(Taskbar)]
pub fn taskbar(props: &TaskbarProps) -> Html {
    let time = use_state(|| current_time());

    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                time.set(current_time());
            });
            move || drop(interval)
        });
    }

    let on_start = {
        let cb = props.on_start_click.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    html! {
        <div class="taskbar win95-panel">
            <button class={classes!("start-button", props.start_menu_open.then_some("active"))} onclick={on_start}>
                { "\u{229E} Start" }
            </button>
            <div class="taskbar-windows">
                { for props.windows.iter().filter(|w| w.open).map(|w| {
                    let game_title = props.games.iter()
                        .find(|g| g.id == w.game_id)
                        .map(|g| g.title.clone())
                        .unwrap_or_else(|| w.game_id.clone());
                    let cb = props.on_focus.clone();
                    let gid = w.game_id.clone();
                    let onclick = Callback::from(move |_: MouseEvent| cb.emit(gid.clone()));
                    html! {
                        <button key={w.game_id.clone()} class="taskbar-window-btn" {onclick}>
                            { game_title }
                        </button>
                    }
                })}
            </div>
            <div class="taskbar-clock">{ (*time).clone() }</div>
        </div>
    }
}
