use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error as ActixError, HttpMessage};
use futures_util::future::LocalBoxFuture;

use crate::{config::Settings, security::UserClaims, error::AppError};

/// Middleware gọi iam-service để introspect token và gắn UserClaims vào Extensions.
pub struct IamAuth { pub cfg: Settings }

impl<S, B> Transform<S, ServiceRequest> for IamAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = IamAuthMw<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(IamAuthMw { service: std::rc::Rc::new(service), cfg: self.cfg.clone() }))
    }
}

pub struct IamAuthMw<S> { service: std::rc::Rc<S>, cfg: Settings }

#[derive(serde::Deserialize)]
struct IntrospectRes {
    active: bool,
    roles: Option<Vec<String>>,      // tuỳ contract iam-service
    permissions: Option<Vec<String>> // tuỳ contract iam-service
}

impl<S, B> Service<ServiceRequest> for IamAuthMw<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let cfg = self.cfg.clone();
        let service = self.service.clone();

        Box::pin(async move {
            let auth = req.headers().get("authorization").and_then(|h| h.to_str().ok()).unwrap_or("");
            let token = auth.strip_prefix("Bearer ").ok_or_else(|| AppError::Unauthorized)?;

            let base = cfg.iam_service_base_url.as_deref().ok_or_else(|| AppError::Unauthorized)?;
            let url = format!("{}/tokens/introspect", base); // đổi nếu contract khác

            let res = reqwest::Client::new()
                .post(url)
                .bearer_auth(token)
                .send()
                .await
                .map_err(|_| AppError::Unauthorized)?;

            if !res.status().is_success() {
                return Err(AppError::Unauthorized.into());
            }
            let info: IntrospectRes = res.json().await.map_err(|_| AppError::Unauthorized)?;
            if !info.active { return Err(AppError::Unauthorized.into()); }

            let claims = UserClaims {
                roles: info.roles.unwrap_or_default(),
                permissions: info.permissions.unwrap_or_default(),
            };
            req.extensions_mut().insert(claims);

            service.call(req).await
        })
    }
}

/// Demo Auth middleware: dùng khi chưa có iam-service (dev local)
pub struct DemoAuth;

impl<S, B> Transform<S, ServiceRequest> for DemoAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = DemoAuthMw<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(DemoAuthMw { service: std::rc::Rc::new(service) }))
    }
}

pub struct DemoAuthMw<S> { service: std::rc::Rc<S> }

impl<S, B> Service<ServiceRequest> for DemoAuthMw<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let claims = UserClaims {
                roles: vec!["ROLE_MASTER_ADMIN".into()],
                permissions: vec![
                    "his.master.code.list".into(),
                    "his.master.code.read".into(),
                    "his.master.code.create".into(),
                    "his.master.code.update".into(),
                    "his.master.code.delete".into(),
                ],
            };
            req.extensions_mut().insert(claims);
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
