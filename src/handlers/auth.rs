use actix_web::{web, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::db::DBPool;
use crate::models::user::{self, NewUser, User};
use crate::schema::users::{self, dsl::*};

#[derive(Deserialize)]
pub struct LoginCredentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    is_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub is_admin: bool,
    pub exp: usize,
}

// Login logic
pub async fn login(
    credentials: web::Json<LoginCredentials>,
    pool: web::Data<DBPool>,
) -> Result<HttpResponse> {
    // get database connection
    let conn = &mut pool.get().unwrap();

    // Find User by username
    let user_result = users
        .filter(username.eq(&credentials.username))
        .first::<User>(conn);

    // login logic
    match user_result {
        Ok(user) => {
            // verify password
            if verify(&credentials.password, &user.password).unwrap() {
                // create JWT Token
                let claims = Claims {
                    sub: user.id,
                    username: user.username,
                    is_admin: user.is_admin,
                    exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(std::env::var("JWT_TOKEN").unwrap().as_bytes()),
                )
                .unwrap();

                Ok(HttpResponse::Ok().json(LoginResponse {
                    token,
                    is_admin: user.is_admin,
                }))
            } else {
                Ok(HttpResponse::Unauthorized().json("Invalid credentials"))
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().json("Invalid credentials")),
    }
}

// Register
pub async fn register (new_user: web::Json<LoginCredentials>, pool: web::Data<DBPool>) -> Result<HttpResponse>{
    // connect to database
    let conn = &mut pool.get().unwrap();

    // check if username already exist
    let exists = users
        .filter(username.eq(&new_user.username))
        .first::<User>(conn)
        .is_ok();

    if exists {
        return Ok(HttpResponse::BadRequest().json("Username already exist!"));
    }

    // hash password
    let password_hash = hash(&new_user, DEFAULT_COST).unwrap();

    let user_struct = NewUser {
        username: new_user.username.clone(),
        password: password_hash,
        is_admin: true,
    };

    // insert new user
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(|_| HttpResponse::InternalServerError().json("Cannot create new user!"))?;

    Ok(HttpResponse::Created().json("User created successfully!"))
}
