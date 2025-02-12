use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use argon2::{self, Argon2};
use argon2::{password_hash::PasswordHash, PasswordVerifier};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHasher,
};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn get_secret_key() -> String {
    dotenv().ok();
    env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not set")
}

pub fn generate_token(user_id: &str) -> String {
    let secret_key = get_secret_key();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .unwrap()
}

pub fn verify_token(token: &str) -> Result<String, String> {
    let secret_key = get_secret_key();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(data) => Ok(data.claims.sub),
        Err(err) => Err(format!("Invalid token: {}", err)),
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    println!("Token received: {}", token);
    match verify_token(token) {
        Ok(user_id) => {
            println!("Token validated for user: {}", user_id);
            req.extensions_mut().insert(user_id);
            Ok(req)
        }
        Err(err) => {
            println!("Token validation failed: {}", err);
            Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
