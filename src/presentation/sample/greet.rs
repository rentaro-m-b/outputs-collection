use axum::http::StatusCode;

pub async fn greet() -> Result<String, StatusCode> {
    Ok("Hello, Axum!".to_owned())
}
