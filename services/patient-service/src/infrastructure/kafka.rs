use serde::Serialize;

#[cfg(feature = "kafka")]
use rdkafka::{config::ClientConfig, producer::{FutureProducer, FutureRecord}};

#[derive(Clone)]
pub struct Kafka {
    #[cfg(feature = "kafka")]
    pub producer: FutureProducer,
}

impl Kafka {
    pub fn new(_brokers: &str, _client_id: &str) -> anyhow::Result<Self> {
        #[cfg(feature = "kafka")]
        {
            let producer = ClientConfig::new()
                .set("bootstrap.servers", _brokers)
                .set("client.id", _client_id)
                .create()?;
            Ok(Self { producer })
        }
        #[cfg(not(feature = "kafka"))]
        {
            anyhow::bail!("Kafka feature not enabled")
        }
    }

    pub async fn publish<T: Serialize>(&self, topic: &str, _key: &str, _payload: &T) -> anyhow::Result<()> {
        #[cfg(feature = "kafka")]
        {
            let bytes = serde_json::to_vec(_payload)?;
            self.producer
                .send(FutureRecord::to(topic).key(_key).payload(&bytes), std::time::Duration::from_secs(0))
                .await
                .map_err(|(e, _)| anyhow::anyhow!(e))?;
            Ok(())
        }
        #[cfg(not(feature = "kafka"))]
        {
            tracing::warn!("Kafka not available, skipping publish to topic: {}", topic);
            Ok(())
        }
    }
}
