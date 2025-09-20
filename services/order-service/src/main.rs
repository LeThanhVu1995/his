// src/main.rs - Using Service Helpers (Simplest)
use app_web::{service_main, prelude::PermissionDef};
use app_auth::keycloak::KeycloakValidator;

mod config;
mod telemetry;
mod error;
mod security;
mod infrastructure;
mod domain;
mod dto;
mod http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Settings::load();
    telemetry::init_tracing(&cfg);

    // One-liner to start everything!
    service_main!(
        service_name: cfg.service_name.clone(),
        config: cfg,
        permission_catalog: |_service_name| {
            vec![
                PermissionDef::new("his.order.list", "List orders", "his.order", "list"),
                PermissionDef::new("his.order.read", "Read order", "his.order", "read"),
                PermissionDef::new("his.order.create", "Create order", "his.order", "create"),
                PermissionDef::new("his.order.update", "Update order", "his.order", "update"),
                PermissionDef::new("his.order.cancel", "Cancel order", "his.order", "cancel"),
                PermissionDef::new("his.order.item.add", "Add order item", "his.order.item", "add"),
                PermissionDef::new("his.order.item.update", "Update order item", "his.order.item", "update"),
                PermissionDef::new("his.order.item.result", "Submit item result", "his.order.item", "result"),
            ]
        },
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: http::mount,
        validator: KeycloakValidator::from_security_config(&cfg.security)
    )
}
