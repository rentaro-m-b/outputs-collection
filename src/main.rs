use tokio::net::TcpListener;
use axum::http::StatusCode;

use axum::{
    routing::get,
    response::{IntoResponse, Response},
    Router
};

#[tokio::main]
async fn main() {
    let app = launch_app();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn launch_app() -> Router {
    Router::new()
        .route("/sample", get(sample_handler))
}

async fn sample_handler() -> Result<String, StatusCode> {
    Ok("Hello, Axum!".to_owned())
}

// AppErrorは中間エラーである。ハンドラーからErrが返されても直ちにエラーとはならない。このエラーを用いて他のエラーをラップする
// struct AppError(anyhow::Error);
//
// fn sample_usecase() -> Result<(), anyhow::Error> {
//     anyhow::bail!("it failed!")
// }

// axumに`AppError`をResponseに変換するメソッドを実装する
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Something went wrong: {}", self.0),
//         )
//             .into_response()
//     }
// }

// AppErrorにFromトレイトを実装する。AppErro::From(e)とすれば AppError{ e: anyhow::Error }を返す
// impl<E> From<E> for AppError
// where
//     E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }
