use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AuthUser(pub Claims);

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // For now, return unauthorized - this would be implemented with proper auth middleware
        ready(Err(crate::error::AppError::Unauthorized.into()))
    }
}
