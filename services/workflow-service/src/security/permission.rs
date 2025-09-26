use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error as ActixError,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;
use crate::error::AppError;
use crate::security::auth_user::AuthUser;

pub struct RequirePermission {
    perm: &'static str,
}

impl RequirePermission {
    pub fn new(p: &'static str) -> Self {
        Self { perm: p }
    }
}

pub struct RequirePermissionMw<S> {
    service: Rc<S>,
    perm: &'static str,
}

impl<S, B> Transform<S, ServiceRequest> for RequirePermission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = RequirePermissionMw<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, s: S) -> Self::Future {
        std::future::ready(Ok(RequirePermissionMw {
            service: Rc::new(s),
            perm: self.perm,
        }))
    }
}

impl<S, B> Service<ServiceRequest> for RequirePermissionMw<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let extensions = req.extensions();
        let user = match extensions.get::<AuthUser>() {
            Some(u) => u,
            None => return Box::pin(async move { Err(AppError::Unauthorized("No authentication found".to_string()).into()) }),
        };
        let needed = self.perm;

        if !user.0.permissions.iter().any(|p| p == needed) {
            return Box::pin(async move { Err(AppError::Forbidden("Insufficient permissions".to_string()).into()) });
        }

        drop(extensions);
        let svc = self.service.clone();
        Box::pin(async move {
            svc.call(req).await
        })
    }
}
