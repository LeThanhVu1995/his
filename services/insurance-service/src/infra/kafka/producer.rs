use app_kafka::KafkaProducer as AkProducer;
use serde::Serialize;

#[derive(Clone)]
pub struct KafkaProducer {
    inner: AkProducer,
}

impl KafkaProducer {
    pub fn from_env(service_name: &str) -> anyhow::Result<Self> {
        let mut cfg = app_config::KafkaConfig::from_env();
        if cfg.client_id.is_empty() {
            cfg.client_id = service_name.to_string();
        }
        let inner = AkProducer::from_config(&cfg, service_name)?;
        Ok(Self { inner })
    }

    pub async fn send_json<T: Serialize>(&self, topic: &str, key: &str, payload: &T) -> anyhow::Result<()> {
        self.inner.send_json(topic, Some(key), payload).await?;
        Ok(())
    }
}
