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
                PermissionDef::new("patient.read", "Read patient data", "patient", "read"),
                PermissionDef::new("patient.create", "Create patient", "patient", "create"),
                PermissionDef::new("patient.update", "Update patient", "patient", "update"),
                PermissionDef::new("encounter.read", "Read encounter data", "encounter", "read"),
                PermissionDef::new("encounter.create", "Create encounter", "encounter", "create"),
                PermissionDef::new("encounter.update", "Update encounter", "encounter", "update"),
                PermissionDef::new("encounter.close", "Close encounter", "encounter", "close"),
            ]
        },
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: http::mount,
        validator: KeycloakValidator::from_security_config(&cfg.security)
    )
}
