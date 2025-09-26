use serde_json::Value as Json;

pub async fn publish(
    topic: &str,
    key: &str,
    payload: &Json,
) -> anyhow::Result<()> {
    tracing::info!("[MOCK] Publishing to Kafka: topic={}, key={}", topic, key);
    tracing::debug!("[MOCK] Payload: {}", serde_json::to_string(payload)?);

    // TODO: Replace with actual Kafka producer when cmake is available
    // For now, just log the message
    Ok(())
}

pub async fn publish_with_headers(
    topic: &str,
    key: &str,
    payload: &Json,
    headers: &std::collections::HashMap<String, String>,
) -> anyhow::Result<()> {
    tracing::info!("[MOCK] Publishing to Kafka with headers: topic={}, key={}", topic, key);
    tracing::debug!("[MOCK] Headers: {:?}", headers);
    tracing::debug!("[MOCK] Payload: {}", serde_json::to_string(payload)?);

    // TODO: Replace with actual Kafka producer when cmake is available
    // For now, just log the message
    Ok(())
}
