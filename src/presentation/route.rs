use axum::{routing::get, Router};
use crate::presentation::sample::greet::greet;

pub fn launch_app() -> Router {
    Router::new()
        .route("/sample", get(greet))
}
