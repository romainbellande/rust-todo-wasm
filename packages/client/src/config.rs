use once_cell::sync::Lazy;
use std::option_env;

pub struct Config {
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_url: option_env!("API_URL").unwrap_or_else(|| "http://127.0.0.1:8080").to_string(),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
