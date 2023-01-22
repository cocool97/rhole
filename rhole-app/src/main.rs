mod api;
mod components;
mod router;

use api::RholeClient;
use lazy_static::lazy_static;
use router::RholeRouter;
use wasm_logger::Config;

lazy_static! {
    static ref RHOLE_CLIENT: RholeClient = RholeClient::new().unwrap();
}

fn main() {
    wasm_logger::init(Config::default());

    yew::Renderer::<RholeRouter>::new().render();
}
