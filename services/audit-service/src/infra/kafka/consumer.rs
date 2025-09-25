use crate::{infra::db::repositories::audit_repo::AuditRepo, domain::entities::audit_event::AuditEvent};

pub async fn run(db: sqlx::Pool<sqlx::Postgres>) {
    // giả lập: ở đây bạn tích hợp lib app-kafka::consumer để subscribe AUDIT_TOPIC
    // khi nhận message JSON -> deserialize thành AuditEvent hoặc DTO tương tự rồi insert.
    let _ = db; // TODO: wire to real consumer
}
