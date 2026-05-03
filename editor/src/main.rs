mod app;
mod components;
pub mod compress;
mod models;
mod routes;
mod services;
pub mod utils;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}
