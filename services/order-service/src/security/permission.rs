use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error as ActixError,
    HttpMessage,
};
use std::{future::Future, pin::Pin, rc::Rc};
use crate::error::AppError;

// If you have an AuthUser in extensions from app_web, you can define it here to read permissions
#[derive(Clone, Debug)]
pub struct Claims {
    pub permissions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AuthUser(pub Claims);

pub struct RequirePermission { perm: &'static str }
impl RequirePermission { pub fn new(p: &'static str) -> Self { Self { perm: p } } }

pub struct RequirePermissionMw<S> { service: Rc<S>, perm: &'static str }

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
        std::future::ready(Ok(RequirePermissionMw { service: Rc::new(s), perm: self.perm }))
    }
}

impl<S, B> Service<ServiceRequest> for RequirePermissionMw<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let extensions = req.extensions();
        // app_web should insert an AuthUser or Claims into extensions; support both
        let allowed = if let Some(u) = extensions.get::<AuthUser>() {
            u.0.permissions.iter().any(|p| p == self.perm)
        } else if let Some(c) = extensions.get::<Claims>() {
            c.permissions.iter().any(|p| p == self.perm)
        } else {
            false
        };

        if !allowed {
            return Box::pin(async move { Err(AppError::Forbidden.into()) });
        }

        drop(extensions);
        let svc = self.service.clone();
        Box::pin(async move { svc.call(req).await })
    }
}


