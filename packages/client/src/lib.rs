#[macro_use]
extern crate log;

mod components;
mod graphql;
mod pages;
mod router;
mod utils;

use components::App;

pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
