// src/consumer.rs placeholder
use std::time::Duration;

use app_config::KafkaConfig;
use app_error::AppError;
use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::TopicPartitionList;
use serde::de::DeserializeOwned;
// use tokio_stream::Stream; // not needed; using rdkafka stream
use tracing::{debug, error, info, instrument, warn};

#[derive(Debug, Clone, Copy)]
pub enum ConsumeOutcome {
    /// Xử lý OK → commit offset
    Commit,
    /// Tạm thời lỗi → không commit (sẽ re-deliver tuỳ chính sách)
    Retry,
    /// Bỏ qua record này nhưng vẫn commit (ví dụ lỗi dữ liệu không thể sửa)
    SkipCommit,
}

pub struct KafkaConsumer {
    inner: StreamConsumer,
    auto_commit: bool,
}

impl KafkaConsumer {
    /// Tạo consumer từ `KafkaConfig`. Dùng `cfg.group_id`; nếu trống → lỗi.
    pub fn from_config(cfg: &KafkaConfig, client_id: &str) -> Result<Self, AppError> {
        if cfg.group_id.trim().is_empty() {
            return Err(AppError::BadRequest(
                "kafka.group_id is required for consumer".into(),
            ));
        }

        let mut cc = ClientConfig::new();
        cc.set("bootstrap.servers", &cfg.brokers);
        cc.set("group.id", &cfg.group_id);
        if !client_id.is_empty() {
            cc.set("client.id", client_id);
        }

        // Reliability defaults
        cc.set("enable.auto.commit", "false"); // mình commit tay
        cc.set("enable.auto.offset.store", "false");
        cc.set("session.timeout.ms", "45000");
        cc.set("auto.offset.reset", "earliest"); // tweak nếu cần
        cc.set("max.poll.interval.ms", "300000");

        if let Some(sec) = &cfg.security {
            super::producer::apply_security(&mut cc, sec);
        }

        let inner = cc
            .create::<StreamConsumer>()
            .map_err(|e| AppError::Upstream(format!("create consumer: {e}")))?;
        Ok(Self {
            inner,
            auto_commit: false,
        })
    }

    /// Đăng ký topics.
    pub fn subscribe(&self, topics: &[&str]) -> Result<(), AppError> {
        self.inner
            .subscribe(topics)
            .map_err(|e| AppError::Upstream(format!("subscribe: {e}")))
    }

    /// Vòng lặp tiêu thụ: gọi `handler` cho mỗi message. Handler trả `ConsumeOutcome`.
    ///
    /// - Commit sẽ thực hiện **sau** khi handler trả `Commit`.
    /// - Với `Retry`: không commit → message sẽ được đọc lại.
    /// - Với `SkipCommit`: bỏ qua message đó, commit để không đọc lại.
    #[instrument(skip_all, fields(topics = ?topics))]
    pub async fn run<F, Fut>(&self, topics: &[&str], mut handler: F) -> Result<(), AppError>
    where
        F: FnMut(&BorrowedMessage<'_>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ConsumeOutcome> + Send,
    {
        self.subscribe(topics)?;
        let mut stream = self.inner.stream();
        while let Some(result) = stream.next().await {
            match result {
                Ok(msg) => {
                    // pass message reference to handler
                    let outcome = handler(&msg).await;
                    match outcome {
                        ConsumeOutcome::Commit | ConsumeOutcome::SkipCommit => {
                            if let Err(e) = self.inner.commit_message(&msg, CommitMode::Async) {
                                error!(error = %e, "commit failed");
                            } else {
                                // committed
                            }
                        }
                        ConsumeOutcome::Retry => {
                            // Không commit → để Kafka re-deliver theo cơ chế group rebalance / poll
                        }
                    }
                }
                Err(e) => {
                    warn!(error = %e, "kafka poll error");
                    // ngủ ngắn để tránh tight loop
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
            }
        }
        Ok(())
    }

    /// Helper: parse payload JSON thành `T`, trả None nếu không có payload hoặc lỗi parse.
    pub fn parse_json<T: DeserializeOwned>(msg: &BorrowedMessage<'_>) -> Option<T> {
        msg.payload().and_then(|b| serde_json::from_slice::<T>(b).ok())
    }

    /// Copy payload bytes into an owned Vec for async processing without lifetime ties
    pub fn payload_bytes(msg: &BorrowedMessage<'_>) -> Option<Vec<u8>> {
        msg.payload().map(|b| b.to_vec())
    }

    /// Truy cập StreamConsumer thô (nếu cần custom).
    pub fn inner(&self) -> &StreamConsumer {
        &self.inner
    }
}
