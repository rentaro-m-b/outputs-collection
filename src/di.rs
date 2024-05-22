use std::sync::Arc;

use crate::infra::provider::Provider;
use crate::application::account::create_user::{CreateUserUsecase, CreateUserUsecaseImpl};
use crate::application::account::authorization::{AuthorizationUsecase, AuthorizationUsecaseImpl};

pub struct DiContainer {
    infra_provider: Provider
}

impl DiContainer {
    pub async fn new() -> Self {
        println!("di container start!");
        let infra_provider = Provider::new()
            .await;
        Self{ infra_provider }
    }
    
    pub fn create_user_usecase(&self) -> Arc<impl CreateUserUsecase> {
        println!("create user usecase start!");
        Arc::new(CreateUserUsecaseImpl {
            user_repository: self.infra_provider.provide_user_repository()
        })
    }

    pub fn authorization_usecase(&self) -> Arc<impl AuthorizationUsecase> {
        println!("login usecase start!");
        Arc::new(AuthorizationUsecaseImpl {})
    }
}

