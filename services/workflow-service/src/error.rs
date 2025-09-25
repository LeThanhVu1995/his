use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "message": msg
                }))
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(json!({
                    "error": "Not Found"
                }))
            }
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Bad Request",
                    "message": msg
                }))
            }
            AppError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Unauthorized",
                    "message": msg
                }))
            }
            AppError::Forbidden(msg) => {
                HttpResponse::Forbidden().json(json!({
                    "error": "Forbidden",
                    "message": msg
                }))
            }
        }
    }
}
