use once_cell::sync::Lazy;
use std::env;
use axum_extra::extract::cookie;

#[derive(Clone, Eq, PartialEq)]
pub enum RustEnv {
    Development,
    Production,
}

impl RustEnv {
    pub fn new(value: String) -> Self {
        match value.as_str() {
            "development" => Self::Development,
            "production" => Self::Production,
            value => panic!("RUST_ENV {} invalid", value),
        }
    }
}

pub struct Config {
    pub jwt_secret: String,
    pub salt: String,
    pub log_level: String,
    pub port: u16,
    pub database_url: String,
    pub client_dir: String,
    pub admin_email: String,
    pub admin_password: String,
    pub rust_env: RustEnv,
    pub access_token_duration: u16,
    pub refresh_token_duration: u32,
    pub cookie_key: cookie::Key,
}

impl Config {
    pub fn new() -> Self {
        let cookie_key = env::var("COOKIE_KEY").expect("COOKIE_KEY must be set");
        let cookie_key_bytes = cookie_key.as_bytes();

        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            salt: env::var("SALT").expect("SALT must be set"),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse::<u16>()
                .expect("PORT is not valid"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            client_dir: env::var("CLIENT_DIR").unwrap_or_else(|_| "../client/dist".to_string()),
            admin_email: env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set"),
            admin_password: env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set"),
            rust_env: RustEnv::new(env::var("RUST_ENV").expect("RUST_ENV must be set")),
            // access token duration in seconds, 10 minutes by default
            access_token_duration: env::var("ACCESS_TOKEN_DURATION").unwrap_or_else(|_| "600".to_string()).parse::<u16>().expect("access token duration is invalid"),
            // refresh token duration in seconds, 1 month by default
            refresh_token_duration: env::var("REFRESH_TOKEN_DURATION").unwrap_or_else(|_| "2592000".to_string()).parse::<u32>().expect("refresh token duration is invalid"),
            cookie_key: cookie::Key::from(cookie_key_bytes),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
