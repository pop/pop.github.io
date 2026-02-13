mod app;
mod components;
mod models;
mod routes;
mod services;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}
