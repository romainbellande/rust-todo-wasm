use tower::{ServiceExt};
use tower_http::services::ServeDir;
use axum::{self, routing::{get, MethodRouter}, body::{boxed, Body}};
use axum::http::{Response, StatusCode};

pub fn serve_client(directory: String) -> MethodRouter {
    get(|req| async move {
        match ServeDir::new(directory).oneshot(req).await {
            Ok(res) => res.map(boxed),
               Err(err) => Response::builder()
                   .status(StatusCode::INTERNAL_SERVER_ERROR)
                   .body(boxed(Body::from(format!("error: {err}"))))
                   .expect("error response"),
        }
    })
}