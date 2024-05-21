use axum::{body::Body, extract::Extension, http::{Request, StatusCode}};
use dotenv::dotenv;
use std::{env, sync::Arc};

use crate::di;
use crate::application::account::login::LoginUsecase;

pub async fn authorization_middleware(
    Extension(di_container): Extension<Arc<di::DiContainer>>,
    req: Request<Body>
) -> Result<Request<Body>, StatusCode> {
    let token = match req.headers().get("Authrorization") {
        Some(value) => match value.to_str() {
            Ok(token) => token.trim_start_matches("Bearer "),
            Err(_) => return Err(StatusCode::UNAUTHORIZED)
        },
        None => return Err(StatusCode::UNAUTHORIZED)
    };

    dotenv().ok();
    let secret_key = &env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let usecase = di_container.login_usecase();
    match usecase.login(token, secret_key).await {
        Ok(_) => Ok(req),
        Err(_) => Err(StatusCode::UNAUTHORIZED)
    }
}
