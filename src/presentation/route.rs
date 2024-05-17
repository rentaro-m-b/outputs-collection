use std::sync::Arc;
use axum::routing::{get, post};
use axum::{Extension, Router};

use crate::di;
use crate::presentation::sample::greet::greet;
use crate::presentation::create_user::create_user as create_user_presentation;

pub async fn launch_app() -> Router {
    let di_container = Arc::new(di::DiContainer::new().await);

    Router::new()
        .route("/sample", get(greet))
        .route("/user", post(create_user_presentation))
        .layer(Extension(di_container))
}
