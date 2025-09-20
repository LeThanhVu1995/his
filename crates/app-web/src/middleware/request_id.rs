// src/middleware/request_id.rs placeholder
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{http::header, Error};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::task::{Context, Poll};
use actix_web::HttpMessage;
use uuid::Uuid;

use app_telemetry::metrics::trace_id;
use tracing::Span;

/// Key lưu trong request extensions.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

pub struct RequestIdMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestIdService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdService { inner: service }))
    }
}

pub struct RequestIdService<S> {
    inner: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdService<S>
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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let rid = req
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        // Put into extensions
        req.extensions_mut().insert(RequestId(rid.clone()));

        // attach to tracing span
        Span::current().record("request_id", &tracing::field::display(&rid));

        let fut = self.inner.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            // If missing, inject header
            let headers = res.headers_mut();
            if !headers.contains_key("x-request-id") {
                headers.insert(
                    header::HeaderName::from_static("x-request-id"),
                    header::HeaderValue::from_str(&rid).unwrap(),
                );
            }
            // If we have OpenTelemetry trace id, echo it
            if let Some(tid) = trace_id() {
                headers.insert(
                    header::HeaderName::from_static("x-trace-id"),
                    header::HeaderValue::from_str(&tid).unwrap_or_else(|_| header::HeaderValue::from_static("invalid")),
                );
            }
            Ok(res)
        })
    }
}
