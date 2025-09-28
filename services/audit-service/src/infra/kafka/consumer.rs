use crate::{infra::db::repositories::audit_repo::AuditRepo, domain::entities::audit_event::AuditEvent};
use crate::infra::kafka::topics;
use app_kafka::KafkaConsumer;
use app_config::KafkaConfig;
use std::env;
use tracing::{info, error, warn};

pub async fn run(db: sqlx::Pool<sqlx::Postgres>) {
    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "audit-service".to_string());
    let kafka_config = KafkaConfig::from_env();

    let consumer = match KafkaConsumer::from_config(&kafka_config, &service_name) {
        Ok(consumer) => consumer,
        Err(e) => {
            error!(?e, "Failed to create Kafka consumer");
            return;
        }
    };

    info!("Starting audit event consumer for topic: {}", topics::AUDIT_TOPIC);

    let topics = [topics::AUDIT_TOPIC];
    if let Err(e) = consumer.run(&topics, move |msg| {
        let db = db.clone();
        let payload = app_kafka::KafkaConsumer::payload_bytes(msg);
        async move {
            match payload {
                Some(payload) => {
                    match serde_json::from_slice::<AuditEvent>(&payload) {
                        Ok(audit_event) => {
                            let repo = AuditRepo { db: &db };
                            match repo.insert(&audit_event).await {
                                Ok(_) => {
                                    info!("Inserted audit event: {}", audit_event.audit_id);
                                    app_kafka::ConsumeOutcome::Commit
                                }
                                Err(e) => {
                                    error!(?e, "Failed to insert audit event");
                                    app_kafka::ConsumeOutcome::Retry
                                }
                            }
                        }
                        Err(e) => {
                            error!(?e, "Failed to deserialize audit event");
                            app_kafka::ConsumeOutcome::SkipCommit
                        }
                    }
                }
                None => {
                    warn!("Received message with no payload");
                    app_kafka::ConsumeOutcome::SkipCommit
                }
            }
        }
    }).await {
        error!(?e, "Consumer run failed");
    }
}
