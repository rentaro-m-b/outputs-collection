use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Json, extract::Extension, response::IntoResponse};
use serde::Deserialize;
use axum::debug_handler;

use crate::application::account::create_user::CreateUserUsecase;
use crate::di;

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
