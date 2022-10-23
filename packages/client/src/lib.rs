#[macro_use]
extern crate log;

mod graphql;
mod components;

use components::App;

pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}