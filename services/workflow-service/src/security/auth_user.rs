use actix_web::{FromRequest, HttpRequest, HttpMessage};
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

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(u) = req.extensions().get::<AuthUser>() {
            return ready(Ok(u.clone()));
        }
        if let Some(c) = req.extensions().get::<Claims>() {
            return ready(Ok(AuthUser(c.clone())));
        }
        ready(Err(crate::error::AppError::Unauthorized("No authentication found".to_string()).into()))
    }
}
