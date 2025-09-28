use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::blood_request::BloodRequest;

pub struct BloodRequestRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> BloodRequestRepo<'a> {
    pub async fn insert(&self, r: &BloodRequest) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO blood_requests(request_id, patient_id, encounter_id, ordering_provider, blood_group, component_code, quantity, priority, indication, status, requested_by, requested_at, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"
        )
        .bind(r.request_id)
        .bind(r.patient_id)
        .bind(r.encounter_id)
        .bind(r.ordering_provider)
        .bind(&r.blood_group)
        .bind(&r.component_code)
        .bind(r.quantity)
        .bind(&r.priority)
        .bind(r.indication.as_ref())
        .bind(&r.status)
        .bind(r.requested_by)
        .bind(r.requested_at)
        .bind(r.created_at)
        .bind(r.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> anyhow::Result<Option<BloodRequest>> {
        Ok(sqlx::query_as::<_, BloodRequest>(
            "SELECT request_id, patient_id, encounter_id, ordering_provider, blood_group, component_code, quantity, priority, indication, status, requested_by, requested_at, created_at, updated_at
             FROM blood_requests WHERE request_id = $1"
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn update_status(&self, id: Uuid, status: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE blood_requests SET status = $2, updated_at = NOW() WHERE request_id = $1")
            .bind(id)
            .bind(status)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
