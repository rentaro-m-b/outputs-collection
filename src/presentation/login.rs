use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Json, extract::Extension, response::IntoResponse, response::Response};
use serde::{Deserialize, Serialize};
use axum::debug_handler;

use crate::application::account::login::LoginUsecase;
use crate::di;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String
}

#[derive(Deserialize, Serialize)]
pub struct MyResponse {
    pub token: String,
}

impl IntoResponse for MyResponse {
    fn into_response(self) -> Response {
        // Response構造体をJson<Response>に変換して返す
        Json(self).into_response()
    }
}

#[debug_handler]
pub async fn login(
    Extension(di_container): Extension<Arc<di::DiContainer>>,
    Json(payload): Json<Request>,
) -> impl IntoResponse {
    let usecase = di_container.login_usecase();
    let email = payload.email;
    let password = payload.password;
    let response = MyResponse {
        token: "token".to_string()
    };
    if let Ok(is_validated) = usecase.login(email, password).await {
        Ok(response)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
