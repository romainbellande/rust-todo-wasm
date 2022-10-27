use axum::http::{Response, StatusCode};
use axum::{
    self,
    body::{boxed, Body},
    routing::{get, MethodRouter},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

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
