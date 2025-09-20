// src/middleware/auth.rs placeholder
use std::rc::Rc;
use std::sync::Arc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{http::header, Error};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::task::{Context, Poll};
use actix_web::HttpMessage;
use app_error::AppError;
use tracing::Span;

use crate::extractors::auth_user::AuthUser;

/// Trait validator token (triển khai ở crate #5, ví dụ Keycloak).
#[allow(async_fn_in_trait)]
pub trait AuthTokenValidator: Send + Sync + Clone + 'static {
    async fn validate(&self, token: &str) -> Result<AuthUser, AppError>;
}

/// Cấu hình bắt buộc scope/role theo route group (đơn giản).
#[derive(Clone, Debug, Default)]
pub struct AuthConfig {
    /// Nếu true, thiếu Authorization vẫn cho qua (user ẩn danh).
    pub optional: bool,
    /// Nếu set, yêu cầu user phải có đủ tất cả scopes.
    pub required_scopes: Vec<String>,
    /// Nếu set, yêu cầu user có ít nhất một role trong danh sách.
    pub any_role: Vec<String>,
}

pub struct AuthMiddleware<V>
where
    V: AuthTokenValidator,
{
    validator: V,
    config: AuthConfig,
}

impl<V> AuthMiddleware<V>
where
    V: AuthTokenValidator,
{
    pub fn new(validator: V, config: AuthConfig) -> Self {
        Self { validator, config }
    }
}

impl<S, B, V> Transform<S, ServiceRequest> for AuthMiddleware<V>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    V: AuthTokenValidator,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthService<S, V>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthService {
            inner: Arc::new(service),
            validator: self.validator.clone(),
            config: self.config.clone(),
        }))
    }
}

pub struct AuthService<S, V>
where
    S: Service<ServiceRequest, Error = Error>,
    V: AuthTokenValidator,
{
    inner: Arc<S>,
    validator: V,
    config: AuthConfig,
}

impl<S, B, V> Service<ServiceRequest> for AuthService<S, V>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    V: AuthTokenValidator,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.as_ref().poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let opt = self.config.optional;

        let header_val = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let validator = self.validator.clone();
        let config = self.config.clone();
        let inner = self.inner.clone();
        let req = req;

        Box::pin(async move {
            let bearer = match header_val {
                Some(h) if h.to_ascii_lowercase().starts_with("bearer ") => {
                    Some(h.split_at(7).1.trim().to_string())
                }
                Some(_) => None,
                None => None,
            };

            if bearer.is_none() && opt {
                return inner.as_ref().call(req).await;
            }

            let token = match bearer {
                Some(t) if !t.is_empty() => t,
                _ => return Err(AppError::Unauthorized.into()),
            };

            let user = validator.validate(&token).await.map_err(|e| {
                match e {
                    AppError::Unauthorized | AppError::Forbidden => e,
                    _ => AppError::Unauthorized,
                }
            })?;

            // Scope/role checks
            if !config.required_scopes.is_empty() {
                let has_all = config
                    .required_scopes
                    .iter()
                    .all(|s| user.scopes.iter().any(|u| u == s));
                if !has_all {
                    return Err(AppError::Forbidden.into());
                }
            }
            if !config.any_role.is_empty() {
                let has_any = config.any_role.iter().any(|r| user.roles.iter().any(|u| u == r));
                if !has_any {
                    return Err(AppError::Forbidden.into());
                }
            }

            // Put user into extensions
            req.extensions_mut().insert(user.clone());

            // Put subject into tracing span
            Span::current().record("user_id", &tracing::field::display(&user.user_id));

            inner.as_ref().call(req).await
        })
    }
}
