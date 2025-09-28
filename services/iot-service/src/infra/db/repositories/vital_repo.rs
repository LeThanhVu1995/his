use sqlx::{Pool, Postgres};
use crate::domain::entities::vital::Vital;

pub struct VitalRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> VitalRepo<'a> {
    pub async fn create(&self, vital: &Vital) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vitals (id, device_id, patient_id, encounter_id, vital_type, value, unit, measured_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(&vital.id)
        .bind(&vital.device_id)
        .bind(&vital.patient_id)
        .bind(&vital.encounter_id)
        .bind(&vital.vital_type)
        .bind(&vital.value)
        .bind(&vital.unit)
        .bind(&vital.measured_at)
        .bind(&vital.created_at)
        .execute(self.db)
        .await?;
        Ok(())
    }
}
