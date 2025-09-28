use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use chrono::Utc;
use serde_json::Value;
use tracing::info;

pub struct S3Exporter {
    client: S3Client,
    bucket: String,
}

impl S3Exporter {
    pub fn new(client: S3Client, bucket: String) -> Self {
        Self { client, bucket }
    }

    pub async fn export_audit_events(&self, events: &[Value]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let key = format!("audit/export_{}.ndjson", timestamp);

        let mut content = String::new();
        for event in events {
            content.push_str(&event.to_string());
            content.push('\n');
        }

        let body = ByteStream::from(content.into_bytes());

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(body)
            .content_type("application/x-ndjson")
            .send()
            .await?;

        info!("Exported {} audit events to S3: s3://{}/{}", events.len(), self.bucket, key);

        Ok(format!("s3://{}/{}", self.bucket, key))
    }

    pub async fn export_audit_events_by_date_range(
        &self,
        events: &[Value],
        start_date: &str,
        end_date: &str
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let key = format!("audit/export_{}_{}_{}.ndjson", start_date, end_date, timestamp);

        let mut content = String::new();
        for event in events {
            content.push_str(&event.to_string());
            content.push('\n');
        }

        let body = ByteStream::from(content.into_bytes());

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(body)
            .content_type("application/x-ndjson")
            .send()
            .await?;

        info!("Exported {} audit events to S3: s3://{}/{}", events.len(), self.bucket, key);

        Ok(format!("s3://{}/{}", self.bucket, key))
    }
}
