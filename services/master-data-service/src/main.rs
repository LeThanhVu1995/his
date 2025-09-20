// src/main.rs - Using Service Helpers (Simplest)
use app_web::{service_main, prelude::PermissionDef};
use app_auth::keycloak::KeycloakValidator;

mod config;
mod telemetry;
mod error;
mod infrastructure;
mod domain;
mod dto;
mod http;
mod security;

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
                PermissionDef::new("master.code.read", "Read master codes", "master.code", "read"),
                PermissionDef::new("master.code.create", "Create master codes", "master.code", "create"),
                PermissionDef::new("master.code.update", "Update master codes", "master.code", "update"),
                PermissionDef::new("master.code.delete", "Delete master codes", "master.code", "delete"),
            ]
        },
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: http::mount,
        validator: KeycloakValidator::from_security_config(&cfg.security)
    )
}
