// src/main.rs - Using Service Helpers (Simplest)
use app_web::service_main;
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
        permission_catalog: crate::security::policy::permission_catalog,
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: http::mount,
        validator: KeycloakValidator::from_security_config(&cfg.security)
    )
}
