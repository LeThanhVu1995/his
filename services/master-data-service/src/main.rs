mod config;
mod telemetry;
mod error;
mod infrastructure;
mod domain;
mod dto;
mod http;
mod security;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer, middleware::Logger, web};
use infrastructure::{db, kafka::Kafka, iam_client};
use app_web::prelude::{AuthMiddleware, AuthConfig};
use app_auth::keycloak::KeycloakValidator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Settings::load();
    telemetry::init_tracing(&cfg);

    // DB
    let pool = db::connect(&cfg.database_url).await.expect("connect db");

    // Sync permission definitions lên IAM-service (Policy-as-code)
    if cfg.iam_service_base_url.is_some() {
        if let Err(e) = iam_client::register_permissions(&cfg).await {
            tracing::warn!(error=?e, "permission register failed");
        } else {
            tracing::info!("permissions registered with IAM");
        }
    }

    // Kafka optional
    let kafka = if let (Some(ref brokers), Some(ref client_id)) = (cfg.kafka_brokers, cfg.kafka_client_id) {
        match Kafka::new(brokers, client_id) {
            Ok(k) => Some(k),
            Err(e) => {
                tracing::warn!(error=?e, "Failed to initialize Kafka, continuing without it");
                None
            }
        }
    } else { None };

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
            // app data
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(kafka.clone()))
            // routes + OpenAPI
            .configure(http::mount)
    })
    .workers(2)
    .bind(host)?
    .run()
    .await
}
