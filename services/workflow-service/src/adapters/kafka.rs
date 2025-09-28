use serde_json::Value as Json;
use once_cell::sync::Lazy;
use std::sync::Arc;

use app_kafka::KafkaProducer as AkProducer;

static PRODUCER: Lazy<anyhow::Result<Arc<AkProducer>>> = Lazy::new(|| {
    // Prefer full AppConfig if available via env; fall back to KafkaConfig::from_env
    let kafka_cfg = app_config::KafkaConfig::from_env();
    let client_id = std::env::var("SERVICE_NAME").unwrap_or_else(|_| "workflow-service".to_string());
    let prod = AkProducer::from_config(&kafka_cfg, &client_id)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(Arc::new(prod))
});

pub async fn publish(
    topic: &str,
    key: &str,
    payload: &Json,
) -> anyhow::Result<()> {
    let producer = std::sync::Arc::clone(PRODUCER.as_ref().as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?);
    producer.send_json(topic, if key.is_empty() { None } else { Some(key) }, payload).await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}

pub async fn publish_with_headers(
    topic: &str,
    key: &str,
    payload: &Json,
    _headers: &std::collections::HashMap<String, String>,
) -> anyhow::Result<()> {
    // app-kafka::KafkaProducer doesn't expose headers in this lightweight API yet.
    // Fallback to sending JSON without headers.
    publish(topic, key, payload).await
}
