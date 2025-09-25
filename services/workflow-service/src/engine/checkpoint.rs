use uuid::Uuid;
use serde_json::Value as Json;
use chrono::{DateTime, Utc};
use crate::store::instances::InstanceStore;

pub struct CheckpointManager<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> CheckpointManager<'a> {
    pub async fn save_checkpoint(
        &self,
        instance_id: Uuid,
        cursor: &Json,
        context: &Json,
        status: &str,
        next_wake: Option<DateTime<Utc>>,
    ) -> anyhow::Result<()> {
        let store = InstanceStore { db: self.db };
        store.save_progress(instance_id, cursor, context, status, next_wake, None).await
    }

    pub async fn restore_checkpoint(&self, instance_id: Uuid) -> anyhow::Result<Option<(Json, Json, String)>> {
        let store = InstanceStore { db: self.db };
        if let Some(instance) = store.get(instance_id).await? {
            Ok(Some((instance.cursor, instance.context, instance.status)))
        } else {
            Ok(None)
        }
    }
}
