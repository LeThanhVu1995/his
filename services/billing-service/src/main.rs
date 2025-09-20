// src/main.rs - Using Service Helpers (Simplest)
use app_web::{service_main, prelude::PermissionDef};
use app_auth::keycloak::KeycloakValidator;

mod config;
mod telemetry;
mod error;
mod security;
mod infra;
mod domain;
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
                PermissionDef::new("his.billing.charge.list", "List charges", "his.billing.charge", "list"),
                PermissionDef::new("his.billing.charge.create", "Create charge", "his.billing.charge", "create"),
                PermissionDef::new("his.billing.charge.update", "Update charge", "his.billing.charge", "update"),
                PermissionDef::new("his.billing.charge.void", "Void charge", "his.billing.charge", "void"),
                PermissionDef::new("his.billing.invoice.list", "List invoices", "his.billing.invoice", "list"),
                PermissionDef::new("his.billing.invoice.read", "Read invoice", "his.billing.invoice", "read"),
                PermissionDef::new("his.billing.invoice.create", "Create invoice", "his.billing.invoice", "create"),
                PermissionDef::new("his.billing.invoice.issue", "Issue invoice", "his.billing.invoice", "issue"),
                PermissionDef::new("his.billing.invoice.void", "Void invoice", "his.billing.invoice", "void"),
                PermissionDef::new("his.billing.payment.list", "List payments", "his.billing.payment", "list"),
                PermissionDef::new("his.billing.payment.create", "Create payment", "his.billing.payment", "create"),
            ]
        },
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: http::mount,
        validator: KeycloakValidator::from_security_config(&cfg.security)
    )
}
