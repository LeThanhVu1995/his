use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::observation::Observation;

pub struct ObservationRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ObservationRepo<'a> {
    pub async fn create(&self, observation: &Observation) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO observation (obs_id, encounter_id, patient_id, device_id, code, value_num, value_text, unit, taken_at, performer_staff_id, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#
        )
        .bind(&observation.obs_id)
        .bind(&observation.encounter_id)
        .bind(&observation.patient_id)
        .bind(&observation.device_id)
        .bind(&observation.code)
        .bind(&observation.value_num)
        .bind(&observation.value_text)
        .bind(&observation.unit)
        .bind(&observation.taken_at)
        .bind(&observation.performer_staff_id)
        .bind(&observation.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, obs_id: Uuid) -> anyhow::Result<Option<Observation>> {
        let observation = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, device_id, code, value_num, value_text, unit, taken_at, performer_staff_id, status
            FROM observation
            WHERE obs_id = $1
            "#
        )
        .bind(obs_id)
        .fetch_optional(self.db)
        .await?;
        Ok(observation)
    }

    pub async fn list_by_patient(&self, patient_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Observation>> {
        let observations = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, device_id, code, value_num, value_text, unit, taken_at, performer_staff_id, status
            FROM observation
            WHERE patient_id = $1
            ORDER BY taken_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(observations)
    }

    pub async fn list_by_encounter(&self, encounter_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Observation>> {
        let observations = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, device_id, code, value_num, value_text, unit, taken_at, performer_staff_id, status
            FROM observation
            WHERE encounter_id = $1
            ORDER BY taken_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(encounter_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(observations)
    }

    pub async fn list_by_code(&self, code: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<Observation>> {
        let observations = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, device_id, code, value_num, value_text, unit, taken_at, performer_staff_id, status
            FROM observation
            WHERE code = $1
            ORDER BY taken_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(code)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(observations)
    }

    pub async fn update(&self, obs_id: Uuid, value_num: Option<rust_decimal::Decimal>, value_text: Option<String>, unit: Option<String>, taken_at: Option<chrono::DateTime<chrono::Utc>>, performer_staff_id: Option<Uuid>, status: Option<String>) -> anyhow::Result<Option<Observation>> {
        let observation = sqlx::query_as::<_, Observation>(
            r#"
            UPDATE observation
            SET value_num = COALESCE($2, value_num),
                value_text = COALESCE($3, value_text),
                unit = COALESCE($4, unit),
                taken_at = COALESCE($5, taken_at),
                performer_staff_id = COALESCE($6, performer_staff_id),
                status = COALESCE($7, status)
            WHERE obs_id = $1
            RETURNING obs_id, encounter_id, patient_id, device_id, code, value_num, value_text, unit, taken_at, performer_staff_id, status
            "#
        )
        .bind(obs_id)
        .bind(value_num)
        .bind(value_text)
        .bind(unit)
        .bind(taken_at)
        .bind(performer_staff_id)
        .bind(status)
        .fetch_optional(self.db)
        .await?;
        Ok(observation)
    }

    pub async fn count_by_patient(&self, patient_id: Uuid) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM observation WHERE patient_id = $1"
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }
}
