use std::sync::Arc;
use axum::http::StatusCode;
use async_trait::async_trait;
use axum::{Json, extract::Extension, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;

use crate::application::account::create_user::{CreateUserUsecase, CreateUserUsecaseImpl};
use crate::presentation::error::create_user::Error as CreateUserApplicationError;

#[derive(Deserialize)]
struct Request {
    username: String,
    email: String,
    password: String
}

pub async fn create_user(
    Json(payload): Json<Request>,
    Extension(create_user_usecase): Extension<Arc<CreateUserUsecaseImpl>>,
) -> Result<impl IntoResponse, StatusCode> {
    let username = payload.username;
    let email = payload.email;
    let password = payload.password;
    if let Ok(_) = create_user_usecase.create_user(username, email, password).await {
        Ok("ok".to_string().into_response())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
