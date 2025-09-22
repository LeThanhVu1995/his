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
        let (s, t, d) = match self {
            Unauthorized => (401, "Unauthorized", None),
            Forbidden => (403, "Forbidden", None),
            NotFound => (404, "Not Found", None),
            BadRequest(m) => (400, "Bad Request", Some(m.clone())),
            Conflict(m) => (409, "Conflict", Some(m.clone())),
            Internal(m) => (500, "Internal Server Error", Some(m.clone())),
        };
        HttpResponse::build(actix_web::http::StatusCode::from_u16(s).unwrap())
            .content_type("application/problem+json")
            .json(ProblemDetails {
                r#type: "about:blank".into(),
                title: t.into(),
                status: s,
                detail: d,
            })
    }
}
