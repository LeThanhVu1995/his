use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error as ActixError, HttpMessage};
use futures_util::future::{LocalBoxFuture, Ready};
use std::rc::Rc;

/// Middleware yêu cầu 1 permission cụ thể.
pub struct RequirePermission {
    perm: &'static str,
}

impl RequirePermission {
    pub fn new(perm: &'static str) -> Self { Self { perm } }
}

impl<S, B> Transform<S, ServiceRequest> for RequirePermission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = RequirePermissionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ready(Ok(RequirePermissionMiddleware {
            service: Rc::new(service),
            perm: self.perm,
        }))
    }
}

pub struct RequirePermissionMiddleware<S> {
    service: Rc<S>,
    perm: &'static str,
}

impl<S, B> Service<ServiceRequest> for RequirePermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let perm = self.perm;
        let svc = self.service.clone();

        Box::pin(async move {
            // Lấy claims đã được middleware xác thực trước đó.
            // Tuỳ tích hợp: có thể dùng `app_auth::Claims` hoặc Extension riêng.
            let has_perm = if let Some(ext) = req.extensions().get::<crate::security::UserClaims>() {
                ext.permissions.iter().any(|p| p == perm)
                    || ext.roles.iter().any(|r| r == "ROLE_MASTER_ADMIN")
            } else {
                false
            };

            if !has_perm {
                use crate::error::AppError;
                return Err(AppError::Forbidden.into());
            }

            svc.call(req).await
        })
    }
}

/// Cấu trúc claims đơn giản hoá. Trong thực tế, trích từ JWT/Keycloak/IAM-service.
#[derive(Clone, Debug)]
pub struct UserClaims {
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
