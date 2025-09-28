// src/store.rs placeholder
use chrono::Utc;
use sqlx::{PgPool, Postgres, Row, Transaction, Executor};
use uuid::Uuid;

use app_error::AppError;
use crate::model::{NewOutboxMsg, OutboxMsg};
pub type PgTxn<'t> = Transaction<'t, Postgres>;

/// Enqueue một outbox message trong **transaction** hiện tại.
/// Trả về `id` đã insert (UUID v4).
pub async fn enqueue(tx: &mut Transaction<'_, Postgres>, msg: &NewOutboxMsg) -> Result<Uuid, AppError> {
    let id = Uuid::new_v4();

    let available_at = msg.available_at.unwrap_or_else(|| Utc::now());

    let q = sqlx::query(
        r#"
        INSERT INTO outbox (
            id, aggregate_type, aggregate_id, event_type,
            topic, partition_key, headers, payload,
            attempts, error, available_at, created_at, delivered_at, locked_at, locked_by
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,0,NULL,$9,now(),NULL,NULL,NULL)
        "#,
    )
    .bind(id)
    .bind(&msg.aggregate_type)
    .bind(&msg.aggregate_id)
    .bind(&msg.event_type)
    .bind(&msg.topic)
    .bind(&msg.partition_key)
    .bind(&msg.headers)
    .bind(&msg.payload)
    .bind(available_at);

    tx.execute(q).await?;

    Ok(id)
}

/// Chọn **batch** bản ghi chưa deliver, set locked_at/locked_by & tăng attempts, rồi **returning** đầy đủ hàng.
/// Sử dụng CTE + `FOR UPDATE SKIP LOCKED` tránh tranh chấp giữa nhiều worker.
pub async fn fetch_batch_for_dispatch(
    pool: &PgPool,
    limit: i64,
    worker_id: &str,
    stale_lock_seconds: i64,
) -> Result<Vec<OutboxMsg>, AppError> {
    let q = sqlx::query(
        r#"
        WITH cte AS (
            SELECT id
            FROM outbox
            WHERE delivered_at IS NULL
              AND available_at <= now()
              AND (locked_at IS NULL OR locked_at < now() - make_interval(secs => $3))
            ORDER BY created_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
        )
        UPDATE outbox o
        SET locked_at = now(),
            locked_by = $2,
            attempts = o.attempts + 1
        FROM cte
        WHERE o.id = cte.id
        RETURNING
            o.id,
            o.aggregate_type,
            o.aggregate_id,
            o.event_type,
            o.topic,
            o.partition_key,
            o.headers,
            o.payload,
            o.attempts,
            o.error,
            o.available_at,
            o.created_at,
            o.delivered_at,
            o.locked_at,
            o.locked_by
        "#,
    )
    .bind(limit)
    .bind(worker_id)
    .bind(stale_lock_seconds);

    let rows = q.fetch_all(pool).await?;

    // Map the raw rows to OutboxMsg structs
    let mut outbox_msgs = Vec::new();
    for row in rows {
        outbox_msgs.push(OutboxMsg {
            id: row.get("id"),
            aggregate_type: row.get("aggregate_type"),
            aggregate_id: row.get("aggregate_id"),
            event_type: row.get("event_type"),
            topic: row.get("topic"),
            partition_key: row.get("partition_key"),
            headers: row.get("headers"),
            payload: row.get("payload"),
            attempts: row.get("attempts"),
            error: row.get("error"),
            available_at: row.get("available_at"),
            created_at: row.get("created_at"),
            delivered_at: row.get("delivered_at"),
            locked_at: row.get("locked_at"),
            locked_by: row.get("locked_by"),
        });
    }

    Ok(outbox_msgs)
}

/// Đánh dấu đã gửi thành công.
pub async fn mark_delivered(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE outbox
        SET delivered_at = now(),
            locked_at = NULL,
            locked_by = NULL,
            error = NULL
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Đánh dấu lỗi và hẹn giờ thử lại lần sau (set available_at theo backoff).
pub async fn mark_failed(
    pool: &PgPool,
    id: Uuid,
    error: &str,
    retry_after: chrono::Duration,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE outbox
        SET error = $2,
            locked_at = NULL,
            locked_by = NULL,
            available_at = now() + $3::interval
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(error)
    .bind(retry_after.num_seconds())
    .execute(pool)
    .await?;
    Ok(())
}
