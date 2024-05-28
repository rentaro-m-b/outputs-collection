use std::sync::Arc;

use crate::application::account::login::{LoginUsecase, LoginUsecaseImpl};
use crate::application::account::service::crypter::{CrypterService, CrypterServiceImpl};
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
            user_repository: self.infra_provider.provide_user_repository(),
            crypter_service: self.crypter_service()
        })
    }

    pub fn authorization_usecase(&self) -> Arc<impl AuthorizationUsecase> {
        println!("authorization usecase start!");
        Arc::new(AuthorizationUsecaseImpl {})
    }

    pub fn login_usecase(&self) -> Arc<impl LoginUsecase> {
        println!("login usecase start!");
        Arc::new(LoginUsecaseImpl {
            user_repository: self.infra_provider.provide_user_repository(),
            crypter_service: self.crypter_service()
        })
    }
    
    fn crypter_service(&self) -> Arc<impl CrypterService> {
        Arc::new(CrypterServiceImpl {
            user_repository: self.infra_provider.provide_user_repository()
        })
    }
}

