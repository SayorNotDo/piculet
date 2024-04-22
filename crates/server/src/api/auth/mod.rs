mod login;

use axum::Router;
use axum::routing::post;

pub fn app() -> Router {
    Router::new()
        .route("/login", post(login::login))
}