use yew::prelude::*;
use crate::config::load_config;
use crate::state::WindowManager;
use crate::components::desktop::Desktop;
use crate::components::window::Window;
use crate::components::game_window::GameWindow;

#[function_component(App)]
pub fn app() -> Html {
    let config = load_config();
    let wm = use_state(|| WindowManager::new(config.games.iter().map(|g| g.id.clone()).collect()));

    let on_open = {
        let wm = wm.clone();
        Callback::from(move |game_id: String| {
            let mut new_wm = (*wm).clone();
            new_wm.z_counter += 1;
            let z = new_wm.z_counter;
            if let Some(w) = new_wm.windows.iter_mut().find(|w| w.game_id == game_id) {
                w.open = true;
                w.z_index = z;
            }
            wm.set(new_wm);
        })
    };

    let open_windows: Vec<_> = wm.windows.iter().filter(|w| w.open).cloned().collect();

    html! {
        <>
            <Desktop games={config.games.clone()} on_open={on_open} />
            { for open_windows.iter().map(|w| {
                let wm_close = wm.clone();
                let game_id_close = w.game_id.clone();
                let wm_focus = wm.clone();
                let game_id_focus = w.game_id.clone();
                let wm_move = wm.clone();
                let game_id_move = w.game_id.clone();

                let on_close = Callback::from(move |_: ()| {
                    let mut new_wm = (*wm_close).clone();
                    if let Some(win) = new_wm.windows.iter_mut().find(|x| x.game_id == game_id_close) {
                        win.open = false;
                    }
                    wm_close.set(new_wm);
                });

                let on_focus = {
                    let wm = wm_focus.clone();
                    let gid = game_id_focus.clone();
                    Callback::from(move |_: ()| {
                        let mut new_wm = (*wm).clone();
                        new_wm.z_counter += 1;
                        let z = new_wm.z_counter;
                        if let Some(win) = new_wm.windows.iter_mut().find(|x| x.game_id == gid) {
                            win.z_index = z;
                        }
                        wm.set(new_wm);
                    })
                };

                let on_move = {
                    let wm = wm_move.clone();
                    let gid = game_id_move.clone();
                    Callback::from(move |pos: (i32, i32)| {
                        let mut new_wm = (*wm).clone();
                        if let Some(win) = new_wm.windows.iter_mut().find(|x| x.game_id == gid) {
                            win.pos = pos;
                        }
                        wm.set(new_wm);
                    })
                };

                let game = config.games.iter().find(|g| g.id == w.game_id).cloned();
                if let Some(game) = game {
                    html! {
                        <Window
                            key={w.game_id.clone()}
                            title={game.title.clone()}
                            z_index={w.z_index}
                            pos={w.pos}
                            on_close={on_close}
                            on_focus={on_focus}
                            on_move={on_move}
                        >
                            <GameWindow game={game} />
                        </Window>
                    }
                } else { html! {} }
            })}
        </>
    }
}
