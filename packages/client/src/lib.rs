#[macro_use]
extern crate log;
extern crate rust_i18n;

mod config;
pub use config::CONFIG;
mod components;
mod errors;
mod graphql;
mod pages;
mod router;
mod store;
mod utils;
use components::App;
pub use errors::Error;
use rust_i18n::i18n;

i18n!("locales");

pub fn start() {
    rust_i18n::set_locale("en");
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
