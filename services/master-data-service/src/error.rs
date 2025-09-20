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

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        use AppError::*;
        let (status, title, detail) = match self {
            Unauthorized => (401, "Unauthorized", None),
            Forbidden => (403, "Forbidden", None),
            NotFound => (404, "Not Found", None),
            Internal(msg) => (500, "Internal Server Error", Some(msg.clone())),
        };
        let body = ProblemDetails {
            r#type: "about:blank".into(),
            title: title.into(),
            status,
            detail,
        };
        HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap())
            .content_type("application/problem+json")
            .json(body)
    }
}
