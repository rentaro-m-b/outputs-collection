use axum::{body::Body, extract::Request, response::Response, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use futures_util::future::BoxFuture;
use tower::{Service, Layer};
use std::task::{Context, Poll};

use crate::di::DiContainer;
use crate::application::account::authorization::AuthorizationUsecase;
use crate::presentation::middleware::error::authorization::Error as AuthorizationMiddlewareError;
use crate::application::account::error::authorization::Error as AuthorizationApplicationError;


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
pub struct AuthorizationMiddleware<S> {
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
        let usecase = self.di_container.authorization_usecase();

        let token = if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_value) = auth_header.to_str() {
                auth_value.trim_start_matches("Bearer ").to_owned()
            } else {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap();
                return Box::pin(async { Ok(response.into_response())})
            }
        } else {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap();
            return Box::pin(async { Ok(response.into_response())})
        };

        let authorize_fut = async move {usecase.authorize(token).await};
        
        let fut = self.inner.call(req);
        Box::pin(async move {
            match authorize_fut.await {
                Ok(_) => println!("authorize ok!"),
                Err(e) => {
                    println!("{}", e);
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap();
                    return Ok(response.into_response());
                }
            };
            let res = fut.await?;
            
            Ok(res)
        })
    }
}
