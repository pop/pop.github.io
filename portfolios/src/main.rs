mod app;
mod components;
mod config;
mod state;
pub mod visual_viewport;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
