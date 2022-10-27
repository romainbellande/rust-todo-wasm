#[macro_use]
extern crate log;

mod components;
mod graphql;

use components::App;

pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
