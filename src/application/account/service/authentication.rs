use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use dotenv::dotenv;
use std::env;

use crate::application::account::service::error::authentication::Error as AuthenticationServiceError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    nbf: usize
}

#[async_trait]
pub trait AuthenticationService: Sync + Send {
    async fn generate_token(&self, user_id: String) -> Result<String, AuthenticationServiceError>;
    async fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, AuthenticationServiceError>;
}

pub struct AuthenticationServiceImpl {}

#[async_trait]
impl AuthenticationService for AuthenticationServiceImpl {
    async fn generate_token(&self, user_id: String) -> Result<String, AuthenticationServiceError> {
        dotenv().ok();
        let secret_key = &env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AuthenticationServiceError::GetCurrentTimeError(e.to_string()))?
            .as_secs() as usize;
        let expiration = now + 3600;
        let claims = Claims {
            sub: user_id,
            exp: expiration,
            nbf: now
        };
    
        let secret_key_bytes = secret_key.as_bytes();
        let header = Header::new(Algorithm::HS256);
    
        Ok(encode(&header, &claims, &EncodingKey::from_secret(secret_key_bytes)).unwrap())
    }
    
    async fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, AuthenticationServiceError> {
        dotenv().ok();
        let secret_key = &env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_nbf = true;
    
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &validation
        );
        token_data.map_err(|e| AuthenticationServiceError::DecordTokenError(e.to_string()))
    }
}


