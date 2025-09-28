use serde::Serialize;
use app_kafka::KafkaProducer as AkProducer;

#[derive(Clone)]
pub struct Kafka {
    producer: AkProducer,
}

impl Kafka {
    pub fn new(_brokers: &str, client_id: &str) -> anyhow::Result<Self> {
        let mut cfg = app_config::KafkaConfig::from_env();
        if !_brokers.is_empty() { cfg.brokers = _brokers.to_string(); }
        if cfg.client_id.is_empty() { cfg.client_id = client_id.to_string(); }
        let producer = AkProducer::from_config(&cfg, client_id)?;
        Ok(Self { producer })
    }

    pub async fn publish<T: Serialize>(&self, topic: &str, key: &str, payload: &T) -> anyhow::Result<()> {
        self.producer.send_json(topic, Some(key), payload).await?;
        Ok(())
    }
}
