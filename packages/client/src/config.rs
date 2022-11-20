use once_cell::sync::Lazy;
use std::option_env;

pub struct Config {
    pub api_url: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let api_url = if let Some(api_url) = option_env!("API_URL") {
            Some(api_url.to_string())
        } else {
            None
        };

        Self { api_url }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
