use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;


// struct for JWT payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub is_admin: bool,
    pub exp: usize,
}

// function generate JWT the token
pub fn create_token(user_id: i32, username: &str, is_admin: bool) -> jsonwebtoken::errors::Result<String> {
    // expiration token
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Invalid timestamp!")
        .timestamp() as usize;

    // implement the JWT struct
    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        is_admin,
        exp: expiration,
    };

    // generate JWT token
    let secret = env::var("JWT_TOKEN").expect("JWT_TOKEN must be set!");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

// funtion to token verification
pub fn verify_token(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    // get the JWT token
    let secret = env::var("JWT_TOKEN").expect("JWT_TOKEN must be set!");

    // validate the token 
    let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

    Ok(decoded.claims)
}
