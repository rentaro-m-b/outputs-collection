use axum::{body::Body, extract::Extension, extract::Request, response::Response, response::IntoResponse, http::StatusCode};
use dotenv::dotenv;
use std::{env, sync::Arc};
use futures_util::future::BoxFuture;
use tower::{Service, Layer};
use std::task::{Context, Poll};

use crate::di::DiContainer;
use crate::application::account::authorization::AuthorizationUsecase;

#[derive(Clone)]
pub struct AuthorizationMiddlewareLayer {
    di_container: Arc<DiContainer>
}

impl AuthorizationMiddlewareLayer {
    pub async fn new(di_container: Arc<DiContainer>) -> Self {
        AuthorizationMiddlewareLayer { di_container }
    }
}

impl<S> Layer<S> for AuthorizationMiddlewareLayer {
    type Service = AuthorizationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizationMiddleware {
            inner,
            di_container: Arc::clone(&self.di_container),
        }
    }
}

#[derive(Clone)]
struct AuthorizationMiddleware<S> {
    inner: S,
    di_container: Arc<DiContainer>
}

impl<S> Service<Request> for AuthorizationMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let fut = self.inner.call(req);
        let token = match req.headers().get("Authrorization") {
            Some(value) => match value.to_str() {
                Ok(token) => token.trim_start_matches("Bearer "),
                Err(_) => {
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap();
                    return Box::pin(async { Ok(response.into_response())});
                }
            },
            None => {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap();
                return Box::pin(async { Ok(response.into_response())});
            }
        };
    
        dotenv().ok();
        let secret_key = &env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let usecase = self.di_container.authorization_usecase();
        
        Box::pin(async move {
            if usecase.login(token, secret_key).await.is_err() {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap();
                return Ok(response.into_response());
            }

            let res = fut.await?;
            Ok(res)
        })
    }
}
