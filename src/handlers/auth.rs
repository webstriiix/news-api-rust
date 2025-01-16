// src/handlers/auth.rs
use actix_web::{web, Error, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DBPool;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::utils::jwt::create_token;

#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    is_admin: bool,
}

pub async fn login(
    credentials: web::Json<LoginCredentials>,
    pool: web::Data<DBPool>,
) -> Result<HttpResponse, Error> {
    let conn = &mut pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    // Find user by username
    let user_result = users
        .filter(username.eq(&credentials.username))
        .first::<User>(conn)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    // Verify password
    if verify(&credentials.password, &user_result.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification failed"))?
    {
        let token = create_token(user_result.id, &user_result.username, user_result.is_admin)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation failed"))?;

        Ok(HttpResponse::Ok().json(LoginResponse {
            token,
            is_admin: user_result.is_admin,
        }))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}

pub async fn register(
    user_data: web::Json<LoginCredentials>,
    pool: web::Data<DBPool>,
) -> Result<HttpResponse, Error> {
    let conn = &mut pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    // Check if username exists
    let user_exists = users
        .filter(username.eq(&user_data.username))
        .first::<User>(conn)
        .is_ok();

    if user_exists {
        return Err(actix_web::error::ErrorBadRequest("Username already exists"));
    }

    // Hash password
    let password_hash = hash(&user_data.password, DEFAULT_COST)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing failed"))?;

    // Create new user
    let new_user = NewUser {
        username: user_data.username.clone(),
        password: password_hash,
        is_admin: false,
    };

    // Insert into database
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create user"))?;

    Ok(HttpResponse::Created().json("User created successfully"))
}
