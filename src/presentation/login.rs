use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Json, extract::Extension, response::IntoResponse, response::Response};
use sea_orm::sea_query::token;
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

    if let Ok(token) = usecase.login(email, password).await {
        let response = MyResponse {
            token: token
        };
        Ok(response.into_response())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
