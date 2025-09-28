use app_web::{service_main, prelude::PermissionDef};
use app_auth::keycloak::KeycloakValidator;

mod config;
mod telemetry;
mod error;
mod security;
mod infrastructure;
mod domain;
mod http;
mod infra;

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
                PermissionDef::new("his.lab.test.list", "List lab tests", "his.lab.test", "list"),
                PermissionDef::new("his.lab.test.create", "Create lab test", "his.lab.test", "create"),
                PermissionDef::new("his.lab.test.update", "Update lab test", "his.lab.test", "update"),
                PermissionDef::new("his.lab.specimen.list", "List specimens", "his.lab.specimen", "list"),
                PermissionDef::new("his.lab.specimen.create", "Create specimen", "his.lab.specimen", "create"),
                PermissionDef::new("his.lab.specimen.collect", "Collect specimen", "his.lab.specimen", "collect"),
                PermissionDef::new("his.lab.specimen.receive", "Receive specimen", "his.lab.specimen", "receive"),
                PermissionDef::new("his.lab.specimen.reject", "Reject specimen", "his.lab.specimen", "reject"),
                PermissionDef::new("his.lab.result.list", "List results", "his.lab.result", "list"),
                PermissionDef::new("his.lab.result.create", "Create result", "his.lab.result", "create"),
                PermissionDef::new("his.lab.result.enter", "Enter result values", "his.lab.result", "enter"),
                PermissionDef::new("his.lab.result.verify", "Verify results", "his.lab.result", "verify"),
                PermissionDef::new("his.lab.result.release", "Release results", "his.lab.result", "release"),
                PermissionDef::new("his.lab.order.create", "Create lab order", "his.lab.order", "create"),
                PermissionDef::new("his.lab.order.get", "Get lab order", "his.lab.order", "get"),
                PermissionDef::new("his.lab.order.list", "List lab orders", "his.lab.order", "list"),
                PermissionDef::new("his.lab.order.update", "Update lab order", "his.lab.order", "update"),
                PermissionDef::new("his.lab.order.delete", "Delete lab order", "his.lab.order", "delete"),
            ]
        },
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: http::mount,
        validator: KeycloakValidator::from_security_config(&cfg.security)
    )
}
