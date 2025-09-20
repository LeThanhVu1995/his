// src/middleware/timeout.rs placeholder
use std::time::Duration;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::task::{Context, Poll};
use tokio::time::timeout;

use app_error::AppError;

#[derive(Clone, Debug)]
pub struct TimeoutMiddleware {
    pub duration: Duration,
}

impl TimeoutMiddleware {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<S, B> Transform<S, ServiceRequest> for TimeoutMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TimeoutService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TimeoutService {
            inner: service,
            duration: self.duration,
        }))
    }
}

pub struct TimeoutService<S> {
    inner: S,
    duration: Duration,
}

impl<S, B> Service<ServiceRequest> for TimeoutService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.inner.call(req);
        let dur = self.duration;
        Box::pin(async move {
            match timeout(dur, fut).await {
                Ok(res) => res,
                Err(_) => Err(AppError::Timeout.into()),
            }
        })
    }
}
