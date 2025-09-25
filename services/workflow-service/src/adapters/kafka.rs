use serde_json::Value as Json;
use app_kafka::producer::KafkaProducer;

pub async fn publish(
    topic: &str,
    key: &str,
    payload: &Json,
) -> anyhow::Result<()> {
    tracing::info!("Publishing to Kafka: topic={}, key={}", topic, key);

    // Get Kafka producer from environment
    let producer = KafkaProducer::from_env()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create Kafka producer: {}", e))?;

    // Serialize payload to bytes
    let payload_bytes = serde_json::to_vec(payload)
        .map_err(|e| anyhow::anyhow!("Failed to serialize payload: {}", e))?;

    // Publish message
    producer
        .send(topic, key, &payload_bytes)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to publish message: {}", e))?;

    tracing::debug!("Successfully published message to topic: {}", topic);
    Ok(())
}

pub async fn publish_with_headers(
    topic: &str,
    key: &str,
    payload: &Json,
    headers: &std::collections::HashMap<String, String>,
) -> anyhow::Result<()> {
    tracing::info!("Publishing to Kafka with headers: topic={}, key={}", topic, key);

    let producer = KafkaProducer::from_env()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create Kafka producer: {}", e))?;

    let payload_bytes = serde_json::to_vec(payload)
        .map_err(|e| anyhow::anyhow!("Failed to serialize payload: {}", e))?;

    // Convert headers to bytes
    let header_bytes: std::collections::HashMap<String, Vec<u8>> = headers
        .iter()
        .map(|(k, v)| (k.clone(), v.as_bytes().to_vec()))
        .collect();

    producer
        .send_with_headers(topic, key, &payload_bytes, &header_bytes)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to publish message with headers: {}", e))?;

    tracing::debug!("Successfully published message with headers to topic: {}", topic);
    Ok(())
}
