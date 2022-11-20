use crate::config::CONFIG;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = CONFIG.jwt_secret.clone();
    Keys::new(secret.as_bytes())
});
