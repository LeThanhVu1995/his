// src/middleware/rate_limit.rs placeholder
use std::num::NonZeroU32;
use std::sync::Arc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::{Error};
use futures_util::future::{ready, LocalBoxFuture, Ready, FutureExt};
use std::task::{Context, Poll};
use actix_web::HttpMessage;
use governor::{
    clock::QuantaClock,
    state::keyed::DashMapStateStore,
    Quota,
    RateLimiter,
};

use app_error::AppError;

/// Cấu hình quota đơn giản
#[derive(Clone, Debug)]
pub struct QuotaConfig {
    pub burst: u32,
    /// Khoảng thời gian (giây) để cấp refill đầy burst.
    pub per_seconds: u64,
}

/// Rule theo method + path prefix
#[derive(Clone, Debug)]
pub struct RateLimitRule {
    pub method: Option<Method>,
    pub path_prefix: String,
    pub quota: QuotaConfig,
}

/// Middleware quản lý nhiều rule theo prefix.
pub struct RateLimitMiddleware {
    rules: Vec<RateLimitRule>,
    /// Keyed limiter theo (key, route)
    limiter: Arc<RateLimiter<(String, String), DashMapStateStore<(String, String)>, QuantaClock>>,
}

impl RateLimitMiddleware {
    pub fn new(rules: Vec<RateLimitRule>) -> Self {
        Self {
            rules,
            limiter: Arc::new(RateLimiter::dashmap(
                Quota::with_period(std::time::Duration::from_secs(1))
                    .unwrap()
                    .allow_burst(NonZeroU32::new(1).unwrap()),
            )),
        }
    }

    fn match_rule(&self, method: &Method, path: &str) -> Option<&RateLimitRule> {
        self.rules.iter().find(|r| {
            (r.method.as_ref().map(|m| m == method).unwrap_or(true))
                && path.starts_with(&r.path_prefix)
        })
    }

    fn client_key(req: &ServiceRequest) -> String {
        // Ưu tiên lấy IP thực (có thể chỉnh theo X-Forwarded-For)
        req.connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string()
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitService {
            inner: Arc::new(service),
            rules: self.rules.clone(),
            limiter: self.limiter.clone(),
        }))
    }
}

pub struct RateLimitService<S> {
    inner: Arc<S>,
    rules: Vec<RateLimitRule>,
    limiter: Arc<RateLimiter<(String, String), DashMapStateStore<(String, String)>, QuantaClock>>,
}

impl<S, B> Service<ServiceRequest> for RateLimitService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.as_ref().poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        let rule = self
            .rules
            .iter()
            .find(|r| (r.method.as_ref().map(|m| m == &method).unwrap_or(true)) && path.starts_with(&r.path_prefix))
            .cloned();

        let limiter = self.limiter.clone();
        let key = (RateLimitMiddleware::client_key(&req), rule.as_ref().map(|r| r.path_prefix.clone()).unwrap_or_else(|| "default".into()));
        let inner = self.inner.clone();
        Box::pin(async move {
            if let Some(r) = rule {
                let burst = NonZeroU32::new(r.quota.burst.max(1)).unwrap();
                let per = std::time::Duration::from_secs(r.quota.per_seconds.max(1));
                let quota = Quota::with_period(per).unwrap().allow_burst(burst);
                let limiter_ref = limiter;
                if limiter_ref.check_key(&key).is_err() {
                    return Err(AppError::TooManyRequests.into());
                }
            }
            // else: nếu không khớp rule nào => bỏ qua limit
            Ok::<(), Error>(())
        }.then(move |_| inner.as_ref().call(req)))
    }
}
