// src/producer.rs placeholder
use std::time::Duration;

use app_config::KafkaConfig;
use app_error::AppError;
use metrics::{describe_counter, increment_counter};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde::Serialize;
use tracing::{debug, error, info, instrument, warn};

/// Kết quả gửi 1 record.
#[derive(Debug, Clone)]
pub struct ProduceReport {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
}

#[derive(Clone)]
pub struct KafkaProducer {
    inner: FutureProducer,
    timeout: Duration,
}

impl KafkaProducer {
    /// Tạo producer từ `KafkaConfig`. `client_id` sẽ set vào config nếu non-empty.
    pub fn from_config(cfg: &KafkaConfig, client_id: &str) -> Result<Self, AppError> {
        let mut cc = ClientConfig::new();
        cc.set("bootstrap.servers", &cfg.brokers);

        if !cfg.group_id.is_empty() {
            // không cần cho producer nhưng một số setup muốn cat log cho client.id
            cc.set("group.id", &cfg.group_id);
        }
        if !client_id.is_empty() {
            cc.set("client.id", client_id);
        }

        // Reliability defaults
        cc.set("message.timeout.ms", "60000");
        cc.set("queue.buffering.max.messages", "100000");
        cc.set("queue.buffering.max.kbytes", "1048576"); // 1GB
        cc.set("compression.type", "lz4");
        cc.set("enable.idempotence", "true"); // exactly-once per producer session
        cc.set("acks", "all");
        cc.set("retries", "10");

        // Security mapping nếu có
        if let Some(sec) = &cfg.security {
            apply_security(&mut cc, sec);
        }

        let inner = cc
            .create::<FutureProducer>()
            .map_err(|e| AppError::Upstream(format!("create producer: {e}")))?;
        Ok(Self {
            inner,
            timeout: Duration::from_secs(10),
        })
    }

    /// Gửi bytes với optional key.
    #[instrument(level = "debug", skip_all, fields(topic = %topic))]
    pub async fn send_bytes(
        &self,
        topic: &str,
        key: Option<&str>,
        payload: &[u8],
    ) -> Result<ProduceReport, AppError> {
        describe_counter!("kafka_produced_total", "Produced messages");
        describe_counter!("kafka_produce_errors_total", "Producer errors");

        let mut rec = FutureRecord::to(topic).payload(payload);
        if let Some(k) = key {
            rec = rec.key(k);
        }

        match self.inner.send(rec, Timeout::After(self.timeout)).await {
            Ok(Ok(delivery)) => {
                increment_counter!("kafka_produced_total", "topic" => topic.to_string());
                let report = ProduceReport {
                    topic: delivery.topic().to_string(),
                    partition: delivery.partition(),
                    offset: delivery.offset(),
                };
                debug!(?report, "kafka delivered");
                Ok(report)
            }
            Ok(Err((e, _))) => {
                increment_counter!("kafka_produce_errors_total", "topic" => topic.to_string());
                error!(error = %e, "kafka delivery failed");
                Err(AppError::Upstream(format!("kafka delivery: {e}")))
            }
            Err(e) => {
                increment_counter!("kafka_produce_errors_total", "topic" => topic.to_string());
                error!(error = %e, "kafka send timeout");
                Err(AppError::Timeout)
            }
        }
    }

    /// Gửi JSON (serialize bằng `serde_json`) — content-type tùy consumer.
    #[instrument(level = "debug", skip_all, fields(topic = %topic))]
    pub async fn send_json<T: Serialize>(
        &self,
        topic: &str,
        key: Option<&str>,
        value: &T,
    ) -> Result<ProduceReport, AppError> {
        let bytes = serde_json::to_vec(value).map_err(|e| AppError::BadRequest(e.to_string()))?;
        self.send_bytes(topic, key, &bytes).await
    }

    /// Thay đổi timeout cho send.
    pub fn with_timeout(mut self, dur: Duration) -> Self {
        self.timeout = dur;
        self
    }
}

/// Parse chuỗi `k=v;...` và set vào ClientConfig.
/// Ví dụ:
/// "security.protocol=SASL_SSL;sasl.mechanism=SCRAM-SHA-256;sasl.username=u;sasl.password=p"
fn apply_security(cfg: &mut ClientConfig, security: &str) {
    for pair in security.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((k, v)) = pair.split_once('=') {
            cfg.set(k.trim(), v.trim());
        }
    }
}
