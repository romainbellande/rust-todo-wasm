use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub jwt_secret: String,
    pub salt: String,
    pub log_level: String,
    pub port: u16,
    pub database_url: String,
    pub client_dir: String,
    pub admin_email: String,
    pub admin_password: String,
}

impl Config {
    pub fn new() -> Self {
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
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
