use std::time::Duration;

use app_kafka::producer::KafkaProducer as AkProducer;

#[derive(Clone)]
pub struct KafkaProducer {
    inner: AkProducer,
}

impl KafkaProducer {
    pub fn from_env() -> anyhow::Result<Self> {
        let cfg = app_config::KafkaConfig::from_env();
        let inner = AkProducer::from_config(&cfg, "search-service")?;
        Ok(Self { inner })
    }

    pub async fn send_json(&self, topic: &str, key: &str, payload: &serde_json::Value) -> anyhow::Result<()> {
        self.inner.send_json(topic, Some(key), payload).await?;
        Ok(())
    }
}


