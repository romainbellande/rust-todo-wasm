use axum::http::HeaderMap;
use axum_extra::extract::cookie::CookieJar;





#[derive(Debug)]
pub struct AccessToken(pub String);

pub fn get_access_token_from_headers(headers: &HeaderMap) -> Option<AccessToken> {
    get_token_from_headers("access_token", headers).map(AccessToken)
}

#[derive(Debug)]
pub struct RefreshToken(pub String);

pub fn get_refresh_token_from_headers(headers: &HeaderMap) -> Option<RefreshToken> {
    get_token_from_headers("refresh_token", headers).map(RefreshToken)
}

pub fn get_token_from_headers(token_name: &'static str, headers: &HeaderMap) -> Option<String> {
    let cookie = CookieJar::from_headers(headers);

    cookie
        .get(token_name)
        .map(|cookie| cookie.value().to_owned())
}
