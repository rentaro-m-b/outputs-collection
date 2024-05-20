use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    nbf: usize
}

pub async fn generate_token(user_id: String, secret_key: &str) -> Result<String, SystemTimeError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;
    let expiration = now + 3600;
    let claims = Claims {
        sub: user_id,
        exp: expiration,
        nbf: now
    };

    let secret_key_bytes = secret_key.as_bytes();

    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key_bytes)).unwrap())
}

pub async fn validate_token(token: &str, secret_key: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::ES256);
    validation.validate_exp = true;
    validation.validate_nbf = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &validation
    );
    token_data
}
