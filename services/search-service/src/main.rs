mod config;
mod telemetry;
mod error;
mod infra;
mod domain;
mod http;
mod security;

use app_web::service_main;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Settings::load();
    telemetry::init_tracing(&cfg);

    // start consumer in background
    tokio::spawn({
        let db = infra::db::pool::connect(&cfg.database_url).await.expect("db");
        async move { infra::kafka::consumer::run(db).await }
    });

    service_main!(
        service_name: cfg.service_name.clone(),
        config: cfg,
        permission_catalog: crate::security::policy::permission_catalog,
        set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
        configure_app: |cfg: &mut actix_web::web::ServiceConfig| {
            cfg.configure(http::mount);
        },
        validator: app_auth::KeycloakValidator::from_security_config(&config::Settings::load().security)
    )
}
