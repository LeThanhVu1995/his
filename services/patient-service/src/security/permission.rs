use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error as ActixError, HttpMessage};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;

pub struct RequirePermission {
    perm: &'static str
}

impl RequirePermission {
    pub fn new(perm: &'static str) -> Self {
        Self { perm }
    }
}

#[derive(Clone, Debug)]
pub struct UserClaims {
    pub sub: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>
}

impl<S, B> Transform<S, ServiceRequest> for RequirePermission
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=ActixError> + 'static,
    S::Future: 'static
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = RequirePermissionMw<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(RequirePermissionMw {
            service: Rc::new(service),
            perm: self.perm
        }))
    }
}

pub struct RequirePermissionMw<S> {
    service: Rc<S>,
    perm: &'static str
}

impl<S, B> Service<ServiceRequest> for RequirePermissionMw<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=ActixError> + 'static,
    S::Future: 'static
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let perm = self.perm;
        Box::pin(async move {
            let allowed = if let Some(c) = req.extensions().get::<UserClaims>() {
                c.permissions.iter().any(|p| p == perm) || c.roles.iter().any(|r| r == "ROLE_PATIENT_ADMIN")
            } else {
                false
            };
            if !allowed {
                return Err(crate::error::AppError::Forbidden.into());
            }
            svc.call(req).await
        })
    }
}
