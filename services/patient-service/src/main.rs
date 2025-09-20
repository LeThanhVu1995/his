mod config;
mod telemetry;
mod error;
mod security;
mod infrastructure;
mod domain;
mod dto;
mod http;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer, middleware::Logger, web};
use infrastructure::{db, iam_client};
use app_web::prelude::{AuthMiddleware, AuthConfig};
use app_auth::keycloak::KeycloakValidator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Settings::load();
    telemetry::init_tracing(&cfg);

    let pool = db::connect(&cfg.database_url).await.expect("db");
    // Kafka disabled for now due to CMake requirement
    // let kafka = if let (Some(b), Some(id)) = (cfg.kafka_brokers.clone(), cfg.kafka_client_id.clone()) {
    //     Some(Kafka::new(&b, &id).expect("kafka"))
    // } else {
    //     None
    // };

    if cfg.iam_service_base_url.is_some() {
        if let Err(e) = iam_client::register_permissions(&cfg).await {
            tracing::warn!(?e, "register perm failed");
        } else {
            tracing::info!("permissions registered");
        }
    }

    let port = cfg.service_port;
    let host = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let governor_conf = GovernorConfigBuilder::default().finish().unwrap();
        let auth_middleware = AuthMiddleware::new(KeycloakValidator::from_security_config(&cfg.security), AuthConfig {
            optional: false,
            required_scopes: vec![],
            any_role: vec![],
        });
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            .wrap(auth_middleware)
            .app_data(web::Data::new(pool.clone()))
            // .app_data(web::Data::new(kafka.clone()))  // Kafka disabled
            .configure(http::mount)
    })
    .workers(2)
    .bind(host)?
    .run()
    .await
}
