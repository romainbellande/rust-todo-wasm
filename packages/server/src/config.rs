use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub jwt_secret: String,
    pub salt: String,
    pub log_level: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            salt: env::var("SALT").expect("SALT must be set"),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string()),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
