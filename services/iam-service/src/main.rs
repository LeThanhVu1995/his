use actix_web::{web, App, HttpServer};
use std::time::Duration;

use app_telemetry::{init_with, TelemetryGuard};
use app_web::prelude::*;
use app_auth::KeycloakValidator;
// use app_kafka::KafkaProducer;  // Disabled due to CMake requirement
// use app_outbox::OutboxDispatcher;  // Disabled due to CMake requirement
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

mod error;
mod config;
mod telemetry; // small glue for reading RUST_LOG
mod http;
mod domain;
mod infra;
mod events;
mod rbac;
mod api_doc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::loader::load_from_env();

    // telemetry
    let _guard: TelemetryGuard = init_with(
        &cfg.service_name,
        Some(&cfg.service_version),
        &cfg.telemetry,
    ).expect("telemetry init");

    // DB
    let pool = infra::db::pool::make_pg_pool(&cfg.database_url)
        .await
        .expect("pg pool");

    // Kafka producer + Outbox dispatcher (disabled due to CMake requirement)
    // let producer = KafkaProducer::from_config(&cfg.kafka, &cfg.service_name)
    //     .expect("kafka producer");
    // let dispatcher = OutboxDispatcher::new(pool.clone(), producer.clone(), format!("{}-outbox", cfg.service_name));
    // actix_rt::spawn(async move {
    //     if let Err(e) = dispatcher.run_forever().await {
    //         tracing::error!(error=%e, "outbox dispatcher stopped");
    //     }
    // });

    // Auth validator (Keycloak)
    let validator = KeycloakValidator::from_security_config(&cfg.security);

    // HTTP
    let host = cfg.http_host.clone();
    let port = cfg.http_port;
    let dev_mode = cfg.dev_mode;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(RequestIdMiddleware)
            .wrap(TimeoutMiddleware::new(Duration::from_secs(30)))
            .wrap(build_cors(dev_mode))
            .service(
                web::scope("/api/iam")
                    // protect API scope
                    .wrap(AuthMiddleware::new(
                        validator.clone(),
                        AuthConfig { optional: false, required_scopes: vec![], any_role: vec![] }
                    ))
                    .configure(http::routes::config_routes)
            )
            // Public endpoints
            .service(SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", crate::api_doc::ApiDoc::openapi()))
            .route("/metrics", web::get().to(app_telemetry::metrics::prometheus_handler))
            .configure(http::routes::public_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
