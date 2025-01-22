use actix_web::{error::ResponseError, http::StatusCode, Error, HttpResponse};
use derive_more::{Display, Error};
use diesel::result::Error as DieselError;

// enum for error object
#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display("Database error: {}", _0)]
    DatabaseError(String),
    #[display("Not Found: {}", _0)]
    NotFoundError(String),
    #[display("Unauthorized: {}", _0)]
    UnauthorizedError(String),
    #[display("Forbidden: {}", _0)]
    ForbiddenError(String),
}

// implement enum
impl ResponseError for AppError {
    // create error message for error response json
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": msg
            })),
            AppError::NotFoundError(msg) => HttpResponse::NotFound().json(json!({
                "error": msg
            })),
            AppError::UnauthorizedError(msg) => HttpResponse::Unauthorized().json(json!({
                "error": msg
            })),
            AppError::ForbiddenError(msg) => HttpResponse::Forbidden().json(json!({
                "error": msg
            })),
        }
    }

    // create error code
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::UnauthorizedError(_) => StatusCode::UNAUTHORIZED,
            AppError::ForbiddenError(_) => StatusCode::FORBIDDEN,
        }
    }
}
