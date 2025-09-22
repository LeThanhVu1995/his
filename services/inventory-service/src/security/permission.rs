use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error as ActixError, HttpMessage,
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
        let svc = self.service.clone();
        let needed = self.perm;

        Box::pin(async move {
            let extensions = req.extensions();
            let user = extensions.get::<AuthUser>()
                .ok_or_else(|| AppError::Unauthorized)?;

            if !user.0.permissions.iter().any(|p| p == needed) {
                return Err(AppError::Forbidden.into());
            }
            drop(extensions);

            svc.call(req).await
        })
    }
}
