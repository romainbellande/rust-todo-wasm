#[macro_use]
extern crate log;

mod config;
pub use config::CONFIG;
mod components;
mod errors;
mod graphql;
mod pages;
mod router;
mod store;
mod utils;
pub use errors::Error;

use components::App;

pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
