mod api;
mod components;

use api::RholeClient;
use components::App;
use lazy_static::lazy_static;
use wasm_logger::Config;

lazy_static! {
    static ref RHOLE_CLIENT: RholeClient = RholeClient::new().unwrap();
}

fn main() {
    wasm_logger::init(Config::default());

    yew::Renderer::<App>::new().render();
}
