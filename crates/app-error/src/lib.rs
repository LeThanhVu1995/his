mod problem_details;

use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

pub use problem_details::ProblemDetails;

/// Lỗi theo field khi validate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Lỗi chung cho toàn bộ service.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation failed")]
    Validation {
        message: String,
        fields: Vec<FieldError>,
    },

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not found: {resource} {id:?}")]
    NotFound {
        resource: String,
        id: Option<String>,
    },

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Upstream timeout")]
    Timeout,

    #[error("Too many requests")]
    TooManyRequests,

    #[error("Upstream error: {0}")]
    Upstream(String),

    #[error("Database error: {0}")]
    Db(String),

    #[error("Internal server error")]
    Internal {
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
        message: Option<String>,
    },
}

impl AppError {
    pub fn internal_msg(msg: impl Into<String>) -> Self {
        Self::Internal {
            cause: None,
            message: Some(msg.into()),
        }
    }

    pub fn internal<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Internal {
            cause: Some(Box::new(err)),
            message: None,
        }
    }

    /// Chuyển `AppError` thành `ProblemDetails`.
    pub fn to_problem_details(
        &self,
        instance: Option<&str>,
        trace_id: Option<&str>,
        correlation_id: Option<&str>,
    ) -> ProblemDetails {
        let status = self.status_code();
        let mut pd = ProblemDetails::new(status);

        let (type_url, title, detail, errors): (&str, &str, Option<String>, Option<Value>) =
            match self {
                AppError::BadRequest(msg) => (
                    "urn:problem-type:bad-request",
                    "Bad Request",
                    Some(msg.clone()),
                    None,
                ),
                AppError::Validation { message, fields } => {
                    let errs = json!(fields);
                    (
                        "urn:problem-type:validation-error",
                        "Validation Error",
                        Some(message.clone()),
                        Some(errs),
                    )
                }
                AppError::Unauthorized => {
                    ("urn:problem-type:unauthorized", "Unauthorized", None, None)
                }
                AppError::Forbidden => ("urn:problem-type:forbidden", "Forbidden", None, None),
                AppError::NotFound { resource, id } => {
                    let d = match id {
                        Some(i) => format!("{} '{}' not found", resource, i),
                        None => format!("{} not found", resource),
                    };
                    ("urn:problem-type:not-found", "Not Found", Some(d), None)
                }
                AppError::Conflict(msg) => (
                    "urn:problem-type:conflict",
                    "Conflict",
                    Some(msg.clone()),
                    None,
                ),
                AppError::Timeout => (
                    "urn:problem-type:timeout",
                    "Upstream Timeout",
                    Some("Upstream did not respond in time".to_string()),
                    None,
                ),
                AppError::TooManyRequests => (
                    "urn:problem-type:too-many-requests",
                    "Too Many Requests",
                    None,
                    None,
                ),
                AppError::Upstream(msg) => (
                    "urn:problem-type:upstream",
                    "Bad Gateway",
                    Some(msg.clone()),
                    None,
                ),
                AppError::Db(msg) => (
                    "urn:problem-type:database",
                    "Database Error",
                    Some(msg.clone()),
                    None,
                ),
                AppError::Internal { message, .. } => (
                    "urn:problem-type:internal",
                    "Internal Server Error",
                    message.clone(),
                    None,
                ),
            };

        pd = pd.with_type(type_url).with_title(title);

        if let Some(d) = detail {
            pd = pd.with_detail(d);
        }
        if let Some(inst) = instance {
            pd = pd.with_instance(inst.to_string());
        }
        if let Some(tid) = trace_id {
            pd = pd.with_trace_id(tid.to_string());
        }
        if let Some(cid) = correlation_id {
            pd = pd.with_correlation_id(cid.to_string());
        }
        if let Some(errs) = errors {
            pd = pd.with_errors(errs);
        }

        pd
    }

    /// Mã HTTP mặc định cho từng loại lỗi.
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Validation { .. } => StatusCode::UNPROCESSABLE_ENTITY, // 422
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Timeout => StatusCode::GATEWAY_TIMEOUT, // 504
            AppError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            AppError::Upstream(_) => StatusCode::BAD_GATEWAY, // 502
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/* ---------- Convenient From conversions ---------- */

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::BadRequest(format!("Invalid JSON: {}", e))
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Internal {
            cause: Some(Box::new(e)),
            message: Some("I/O error".into()),
        }
    }
}

#[cfg(feature = "sqlx")]
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound {
                resource: "resource".into(),
                id: None,
            },
            _ => AppError::Db(e.to_string()),
        }
    }
}

#[cfg(feature = "actix")]
mod actix_impl {
    use super::*;
    use actix_web::{http::header, HttpResponse, ResponseError};

    impl ResponseError for AppError {
        fn status_code(&self) -> actix_web::http::StatusCode {
            actix_web::http::StatusCode::from_u16(super::AppError::status_code(self).as_u16())
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }

        fn error_response(&self) -> HttpResponse {
            let pd = self.to_problem_details(None, None, None);
            HttpResponse::build(actix_web::http::StatusCode::from_u16(self.status_code().as_u16()).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))
                .insert_header((header::CONTENT_TYPE, ProblemDetails::CONTENT_TYPE))
                .json(pd)
        }
    }
}
