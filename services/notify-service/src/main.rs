mod config;
mod telemetry;
mod error;
mod security;
mod infrastructure;
mod domain;
mod dto;
mod http;

use app_web::service_main;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Settings::load();
    telemetry::init_tracing(&cfg);

    service_main!(
        service_name: cfg.service_name.clone(),
        config: cfg,
        permission_catalog: crate::security::policy::permission_catalog,
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: |cfg: &mut actix_web::web::ServiceConfig| {
            cfg.configure(http::mount);
        },
        validator: app_auth::KeycloakValidator::from_security_config(&cfg.security)
    )
}
