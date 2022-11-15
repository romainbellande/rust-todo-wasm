use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_url: env!("API_URL", "API_URL is required").to_string(),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
