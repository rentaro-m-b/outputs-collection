use std::sync::Arc;
use axum::routing::{get, post};
use axum::{Extension, Router};

use crate::presentation::sample::greet::greet;
use crate::infra::db::launch_conn;
use crate::infra::repository::user::UserRepositoryImpl;
use crate::application::account::create_user::CreateUserUsecaseImpl;
use crate::presentation::create_user::create_user as create_user_presentation;

pub async fn launch_app() -> Router {
    let db_conn = Arc::new(launch_conn().await.expect("Connection error"));
    let user_repository = Arc::new(UserRepositoryImpl { conn: db_conn });
    let create_user_usecase = Arc::new(CreateUserUsecaseImpl { user_repository: user_repository });

    Router::new()
        .route("/sample", get(greet))
        .route("/user", post(create_user_presentation))
}
