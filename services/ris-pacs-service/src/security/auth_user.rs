use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: String,
    pub username: String,
    pub permissions: Vec<String>,
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // For now, return unauthorized - this would be implemented with proper auth middleware
        ready(Err(crate::error::AppError::Unauthorized.into()))
    }
}
