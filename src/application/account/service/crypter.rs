use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub async fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password_bytes, &salt)?.to_string())
}

pub async fn parse_password_hash(password_hash: &str) -> Result<String, argon2::password_hash::Error> {
    Ok(PasswordHash::new(password_hash)?.to_string())
}

pub async fn validate_password(password: &str, password_hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let password_bytes = password.as_bytes();
    let parsed_password_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default().verify_password(password_bytes, &parsed_password_hash).is_ok())
}
