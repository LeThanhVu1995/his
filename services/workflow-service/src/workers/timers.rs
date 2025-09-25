use chrono::Utc;
use sqlx::Row;
use crate::store::instances::InstanceStore;

pub async fn poll_and_wake(db: &sqlx::Pool<sqlx::Postgres>) -> anyhow::Result<Vec<uuid::Uuid>> {
    let store = InstanceStore { db };
    store.list_waiting().await
}
