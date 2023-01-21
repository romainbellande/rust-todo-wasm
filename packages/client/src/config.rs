use once_cell::sync::Lazy;
use std::option_env;

#[derive(Default)]
pub struct Config {
    pub api_url: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let api_url = option_env!("API_URL").map(|api_url| api_url.to_string());

        Self { api_url }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
