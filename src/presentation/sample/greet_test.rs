use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use tower::ServiceExt;
use crate::presentation::route::launch_app;

#[tokio::test]
async fn greet() {
    let app = launch_app();

    let response = app
        .oneshot(Request::builder().uri("/sample").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, Axum!");
}
