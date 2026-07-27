use yew::prelude::*;
use gloo_timers::callback::Interval;
use crate::config::Game;
use crate::state::WindowState;

#[derive(Properties, PartialEq)]
pub struct TaskbarProps {
    pub windows: Vec<WindowState>,
    pub projects: Vec<Game>,
    pub on_focus: Callback<String>,
    pub on_start_click: Callback<()>,
    pub start_menu_open: bool,
    pub start_icon: String,
    pub start_label: String,
    /// If `Some`, show a Webamp tray icon; the bool is whether Webamp is active.
    #[prop_or_default]
    pub webamp_tray: Option<bool>,
    #[prop_or_default]
    pub on_webamp_toggle: Callback<()>,
}

fn is_url(s: &str) -> bool {
    s.starts_with("http") || s.starts_with("data:") || s.contains('/') || s.contains('.')
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
    let time = use_state(current_time);

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

    let on_webamp_toggle = {
        let cb = props.on_webamp_toggle.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    html! {
        <div class="taskbar win95-panel">
            <button class={classes!("start-button", props.start_menu_open.then_some("active"))} onclick={on_start}>
                if is_url(&props.start_icon) {
                    <img src={props.start_icon.clone()} alt="" class="start-button-icon" />
                } else if !props.start_icon.is_empty() {
                    <span class="start-button-icon">{ &props.start_icon }</span>
                }
                { &props.start_label }
            </button>
            <div class="taskbar-windows">
                { for props.windows.iter().filter(|w| w.open).map(|w| {
                    let game_title = props.projects.iter()
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
            if let Some(active) = props.webamp_tray {
                <button
                    class={classes!("tray-icon-btn", active.then_some("active"))}
                    onclick={on_webamp_toggle}
                    title="Winamp"
                >
                    <img src="wm-4.png" alt="Winamp" class="tray-icon-img" />
                </button>
            }
            <div class="taskbar-clock">{ (*time).clone() }</div>
        </div>
    }
}
