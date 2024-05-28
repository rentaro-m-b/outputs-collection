use std::sync::Arc;
use axum::routing::{get, post};
use axum::{Extension, Router};

use crate::di;
use crate::presentation::sample::greet::greet;
use crate::presentation::create_user::create_user as create_user_presentation;
use crate::presentation::login::login as login_presentation;
use crate::presentation::middleware::authorization::AuthorizationMiddlewareLayer;

pub async fn launch_app() -> Router {
    let di_container = Arc::new(di::DiContainer::new().await);

    Router::new()
        .nest("/sample", Router::new()
            .route("/", get(greet))
            .layer(AuthorizationMiddlewareLayer::new(di_container.clone()).await)
        )
        .nest("/user", Router::new()
            .route("/register", post(create_user_presentation))
            .route("/login", post(login_presentation))
        )
        .layer(Extension(di_container.clone()))
}
