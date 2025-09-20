// src/dispatcher.rs placeholder
use std::time::Duration;

use chrono::Duration as ChronoDuration;
use metrics::{describe_counter, increment_counter};
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

use app_error::AppError;
use app_kafka::KafkaProducer;

use crate::model::OutboxMsg;
use crate::store::{fetch_batch_for_dispatch, mark_delivered, mark_failed, PgPool};

/// Cấu hình backoff retry đơn giản theo số lần attempt.
#[derive(Debug, Clone)]
pub struct BackoffPolicy {
    pub min: ChronoDuration,
    pub step: ChronoDuration,
    pub max: ChronoDuration,
}

impl Default for BackoffPolicy {
    fn default() -> Self {
        Self {
            min: ChronoDuration::seconds(30),  // lần 1: 30s
            step: ChronoDuration::minutes(2),  // cộng dồn 2' mỗi lần
            max: ChronoDuration::minutes(30), // trần 30'
        }
    }
}

impl BackoffPolicy {
    pub fn for_attempt(&self, attempts: i32) -> ChronoDuration {
        let mut delay = self.min + self.step * (attempts.max(1) as i32 - 1);
        if delay > self.max {
            delay = self.max;
        }
        if delay < ChronoDuration::seconds(1) {
            delay = ChronoDuration::seconds(1);
        }
        delay
    }
}

/// Worker đọc outbox và publish sang Kafka.
#[derive(Clone)]
pub struct OutboxDispatcher {
    pool: PgPool,
    producer: KafkaProducer,
    worker_id: String,
    batch_size: i64,
    stale_lock_seconds: i64,
    interval: Duration,
    backoff: BackoffPolicy,
}

impl OutboxDispatcher {
    pub fn new(
        pool: PgPool,
        producer: KafkaProducer,
        worker_id: impl Into<String>,
    ) -> Self {
        Self {
            pool,
            producer,
            worker_id: worker_id.into(),
            batch_size: 100,
            stale_lock_seconds: 300, // 5 phút
            interval: Duration::from_millis(500),
            backoff: BackoffPolicy::default(),
        }
    }

    pub fn with_batch_size(mut self, n: i64) -> Self { self.batch_size = n; self }
    pub fn with_interval(mut self, d: Duration) -> Self { self.interval = d; self }
    pub fn with_stale_lock(mut self, secs: i64) -> Self { self.stale_lock_seconds = secs; self }
    pub fn with_backoff(mut self, b: BackoffPolicy) -> Self { self.backoff = b; self }

    /// Chạy vòng lặp vô hạn (spawn trong Tokio).
    pub async fn run_forever(&self) -> Result<(), AppError> {
        describe_counter!("outbox_fetched_total", "Outbox rows fetched for dispatch");
        describe_counter!("outbox_dispatched_total", "Outbox rows dispatched successfully");
        describe_counter!("outbox_failed_total", "Outbox dispatch failed");
        describe_counter!("outbox_requeued_total", "Outbox rows requeued (set available_at)");

        loop {
            let n = self.run_once().await?;
            if n == 0 {
                sleep(self.interval).await;
            }
        }
    }

    /// Xử lý một batch; trả về số bản ghi đã kéo về (dù thành công hay thất bại).
    #[instrument(skip_all, fields(worker_id = %self.worker_id))]
    pub async fn run_once(&self) -> Result<usize, AppError> {
        let batch = fetch_batch_for_dispatch(
            &self.pool,
            self.batch_size,
            &self.worker_id,
            self.stale_lock_seconds,
        )
        .await?;

        if batch.is_empty() {
            return Ok(0);
        }
        increment_counter!("outbox_fetched_total", "worker" => self.worker_id.clone());

        for msg in batch {
            if let Err(e) = self.dispatch_one(&msg).await {
                // Lỗi gửi → mark_failed với backoff
                let delay = self.backoff.for_attempt(msg.attempts);
                let err_text = truncate_err(&e, 4000);
                if let Err(e2) = mark_failed(&self.pool, msg.id, &err_text, delay).await {
                    error!(error = %e2, "mark_failed error");
                } else {
                    increment_counter!("outbox_requeued_total", "topic" => msg.topic.clone());
                }
            } else {
                if let Err(e2) = mark_delivered(&self.pool, msg.id).await {
                    error!(error = %e2, "mark_delivered error");
                } else {
                    increment_counter!("outbox_dispatched_total", "topic" => msg.topic.clone());
                }
            }
        }

        Ok(batch.len())
    }

    /// Gửi 1 msg sang Kafka (bytes JSON + optional key).
    async fn dispatch_one(&self, msg: &OutboxMsg) -> Result<(), AppError> {
        let key = msg.effective_key();
        let bytes = msg.payload_bytes();

        // (Optional) bạn có thể merge headers vào payload hoặc set vào Kafka headers (rdkafka)
        // Ở đây đơn giản chỉ gửi payload JSON
        self.producer
            .send_bytes(&msg.topic, key, &bytes)
            .await
            .map(|_| ())
            .map_err(|e| {
                increment_counter!("outbox_failed_total", "topic" => msg.topic.clone());
                e
            })
    }
}

fn truncate_err(e: &AppError, max: usize) -> String {
    let s = e.to_string();
    if s.len() > max { s[..max].to_string() } else { s }
}
