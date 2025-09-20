use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ProblemDetails {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub detail: Option<String>,
}

impl ProblemDetails {
    pub const CONTENT_TYPE: &'static str = "application/problem+json";
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        use AppError::*;
        let (status, title, detail) = match self {
            Unauthorized => (401, "Unauthorized", None),
            Forbidden => (403, "Forbidden", None),
            NotFound => (404, "Not Found", None),
            BadRequest(msg) => (400, "Bad Request", Some(msg.clone())),
            Conflict(msg) => (409, "Conflict", Some(msg.clone())),
            Internal(msg) => (500, "Internal Server Error", Some(msg.clone())),
        };
        let body = ProblemDetails {
            r#type: "about:blank".into(),
            title: title.into(),
            status,
            detail
        };
        HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap())
            .insert_header((actix_web::http::header::CONTENT_TYPE, ProblemDetails::CONTENT_TYPE))
            .json(body)
    }
}
