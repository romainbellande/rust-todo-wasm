use super::Claims;
use axum::{
    extract::{FromRequest, RequestParts},
    http::Request,
    middleware::Next,
    response::IntoResponse,
};

pub async fn auth_bearer_middleware<B: std::marker::Send>(
    req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    let mut request_parts = RequestParts::new(req);
    let result = Claims::from_request(&mut request_parts).await;

    match result {
        Ok(claims) => {
            format!("Found claims: {:?}", claims);

            let mut req = request_parts.try_into_request().expect("body extracted");

            req.extensions_mut().insert(claims);

            Ok(next.run(req).await)
        }
        Err(web_error) => Err(web_error),
    }
}
