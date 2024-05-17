use std::sync::Arc;
use axum::http::StatusCode;
use async_trait::async_trait;
use axum::{Json, extract::Extension, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use axum::debug_handler;

use crate::application::account::create_user::{CreateUserUsecase, CreateUserUsecaseImpl};
use crate::di;
use crate::presentation::error::create_user::Error as CreateUserApplicationError;

#[derive(Deserialize)]
pub struct Request {
    username: String,
    email: String,
    password: String
}

#[debug_handler]
pub async fn create_user(
    Extension(di_container): Extension<Arc<di::DiContainer>>,
    Json(payload): Json<Request>,
) -> impl IntoResponse {
    let usecase = di_container.create_user_usecase();
    let username = payload.username;
    let email = payload.email;
    let password = payload.password;
    if let Ok(_) = usecase.create_user(username, email, password).await {
        Ok("ok".to_string().into_response())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
