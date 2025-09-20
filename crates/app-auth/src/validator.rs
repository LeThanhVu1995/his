// src/validator.rs placeholder
use app_error::AppError;
use app_web::prelude::AuthUser;

#[allow(async_fn_in_trait)]
pub trait JwtValidator {
    /// Validate a bearer token and return AuthUser (normalized)
    async fn validate(&self, token: &str) -> Result<AuthUser, AppError>;
}
