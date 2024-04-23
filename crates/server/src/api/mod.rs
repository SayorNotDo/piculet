mod auth;

use axum::routing::*;

pub async fn health() -> &'static str {
    "Hello Piculet!"
}

pub fn app() -> Router {
    Router::new()
        // .route("/health", get(health()))
}