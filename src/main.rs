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
                PermissionDef::new("his.pharmacy.med.list", "List medications", "his.pharmacy.med", "list"),
                PermissionDef::new("his.pharmacy.med.create", "Create medication", "his.pharmacy.med", "create"),
                PermissionDef::new("his.pharmacy.med.update", "Update medication", "his.pharmacy.med", "update"),
                PermissionDef::new("his.pharmacy.presc.list", "List prescriptions", "his.pharmacy.presc", "list"),
                PermissionDef::new("his.pharmacy.presc.read", "Read prescription", "his.pharmacy.presc", "read"),
                PermissionDef::new("his.pharmacy.presc.create", "Create prescription", "his.pharmacy.presc", "create"),
                PermissionDef::new("his.pharmacy.presc.update", "Update prescription", "his.pharmacy.presc", "update"),
                PermissionDef::new("his.pharmacy.presc.approve", "Approve prescription", "his.pharmacy.presc", "approve"),
                PermissionDef::new("his.pharmacy.presc.cancel", "Cancel prescription", "his.pharmacy.presc", "cancel"),
                PermissionDef::new("his.pharmacy.disp.list", "List dispenses", "his.pharmacy.disp", "list"),
                PermissionDef::new("his.pharmacy.disp.create", "Create dispense", "his.pharmacy.disp", "create"),
                PermissionDef::new("his.pharmacy.disp.finish", "Finish dispense", "his.pharmacy.disp", "finish"),
            ]
        },
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: http::mount,
        validator: KeycloakValidator::from_security_config(&cfg.security)
    )
}
