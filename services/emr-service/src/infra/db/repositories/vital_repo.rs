use anyhow::Result;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::vital::{VitalSignRecord, VitalSignItem, Observation};

pub struct VitalRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> VitalRepo<'a> {
    // Vital Sign Record CRUD operations
    pub async fn create_vital_record(&self, record: &VitalSignRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vital_sign_record (
                vs_id, encounter_id, patient_id, measured_at, recorder_staff_id, note
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(&record.vs_id)
        .bind(&record.encounter_id)
        .bind(&record.patient_id)
        .bind(&record.measured_at)
        .bind(&record.recorder_staff_id)
        .bind(&record.note)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_vital_record(&self, record_id: &str) -> Result<Option<VitalSignRecord>> {
        let record = sqlx::query_as::<_, VitalSignRecord>(
            r#"
            SELECT vs_id, encounter_id, patient_id, measured_at, recorder_staff_id, note
            FROM vital_sign_record
            WHERE vs_id = $1
            "#
        )
        .bind(record_id)
        .fetch_optional(self.db)
        .await?;
        Ok(record)
    }

    pub async fn list_patient_vital_records(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<VitalSignRecord>> {
        let records = sqlx::query_as::<_, VitalSignRecord>(
            r#"
            SELECT vs_id, encounter_id, patient_id, measured_at, recorder_staff_id, note
            FROM vital_sign_record
            WHERE patient_id = $1
            ORDER BY measured_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(records)
    }

    pub async fn list_encounter_vital_records(&self, encounter_id: &str, limit: i64, offset: i64) -> Result<Vec<VitalSignRecord>> {
        let records = sqlx::query_as::<_, VitalSignRecord>(
            r#"
            SELECT vs_id, encounter_id, patient_id, measured_at, recorder_staff_id, note
            FROM vital_sign_record
            WHERE encounter_id = $1
            ORDER BY measured_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(encounter_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(records)
    }

    pub async fn update_vital_record(&self, record: &VitalSignRecord) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE vital_sign_record SET
                encounter_id = $2, patient_id = $3, measured_at = $4,
                recorder_staff_id = $5, note = $6
            WHERE vs_id = $1
            "#
        )
        .bind(&record.vs_id)
        .bind(&record.encounter_id)
        .bind(&record.patient_id)
        .bind(&record.measured_at)
        .bind(&record.recorder_staff_id)
        .bind(&record.note)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_vital_record(&self, record_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM vital_sign_record
            WHERE vs_id = $1
            "#
        )
        .bind(record_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_patient_vital_records(&self, patient_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM vital_sign_record
            WHERE patient_id = $1
            "#
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_encounter_vital_records(&self, encounter_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM vital_sign_record
            WHERE encounter_id = $1
            "#
        )
        .bind(encounter_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    // Vital Sign Item operations
    pub async fn create_vital_item(&self, item: &VitalSignItem) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vital_sign_item (
                vs_item_id, vs_id, code, value_num, value_text, unit
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(&item.vs_item_id)
        .bind(&item.vs_id)
        .bind(&item.code)
        .bind(&item.value_num)
        .bind(&item.value_text)
        .bind(&item.unit)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_vital_items(&self, record_id: &str) -> Result<Vec<VitalSignItem>> {
        let items = sqlx::query_as::<_, VitalSignItem>(
            r#"
            SELECT vs_item_id, vs_id, code, value_num, value_text, unit
            FROM vital_sign_item
            WHERE vs_id = $1
            ORDER BY code
            "#
        )
        .bind(record_id)
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn get_vital_item(&self, item_id: &str) -> Result<Option<VitalSignItem>> {
        let item = sqlx::query_as::<_, VitalSignItem>(
            r#"
            SELECT vs_item_id, vs_id, code, value_num, value_text, unit
            FROM vital_sign_item
            WHERE vs_item_id = $1
            "#
        )
        .bind(item_id)
        .fetch_optional(self.db)
        .await?;
        Ok(item)
    }

    pub async fn update_vital_item(&self, item: &VitalSignItem) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE vital_sign_item SET
                vs_id = $2, code = $3, value_num = $4, value_text = $5, unit = $6
            WHERE vs_item_id = $1
            "#
        )
        .bind(&item.vs_item_id)
        .bind(&item.vs_id)
        .bind(&item.code)
        .bind(&item.value_num)
        .bind(&item.value_text)
        .bind(&item.unit)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_vital_item(&self, item_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM vital_sign_item
            WHERE vs_item_id = $1
            "#
        )
        .bind(item_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    // Observation CRUD operations
    pub async fn create_observation(&self, observation: &Observation) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO observation (
                obs_id, encounter_id, patient_id, code, value_num, value_text,
                unit, taken_at, performer_staff_id, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(&observation.obs_id)
        .bind(&observation.encounter_id)
        .bind(&observation.patient_id)
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

    pub async fn get_observation(&self, observation_id: &str) -> Result<Option<Observation>> {
        let observation = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, code, value_num, value_text,
                   unit, taken_at, performer_staff_id, status
            FROM observation
            WHERE obs_id = $1
            "#
        )
        .bind(observation_id)
        .fetch_optional(self.db)
        .await?;
        Ok(observation)
    }

    pub async fn list_patient_observations(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<Observation>> {
        let observations = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, code, value_num, value_text,
                   unit, taken_at, performer_staff_id, status
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

    pub async fn list_encounter_observations(&self, encounter_id: &str, limit: i64, offset: i64) -> Result<Vec<Observation>> {
        let observations = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, code, value_num, value_text,
                   unit, taken_at, performer_staff_id, status
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

    pub async fn list_observations_by_type(&self, observation_type: &str, limit: i64, offset: i64) -> Result<Vec<Observation>> {
        let observations = sqlx::query_as::<_, Observation>(
            r#"
            SELECT obs_id, encounter_id, patient_id, code, value_num, value_text,
                   unit, taken_at, performer_staff_id, status
            FROM observation
            WHERE code = $1
            ORDER BY taken_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(observation_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(observations)
    }

    pub async fn update_observation(&self, observation: &Observation) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE observation SET
                encounter_id = $2, patient_id = $3, code = $4, value_num = $5,
                value_text = $6, unit = $7, taken_at = $8, performer_staff_id = $9, status = $10
            WHERE obs_id = $1
            "#
        )
        .bind(&observation.obs_id)
        .bind(&observation.encounter_id)
        .bind(&observation.patient_id)
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

    pub async fn delete_observation(&self, observation_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE observation SET
                status = 'deleted'
            WHERE obs_id = $1
            "#
        )
        .bind(observation_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_patient_observations(&self, patient_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM observation
            WHERE patient_id = $1 AND status != 'deleted'
            "#
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_encounter_observations(&self, encounter_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM observation
            WHERE encounter_id = $1 AND status != 'deleted'
            "#
        )
        .bind(encounter_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_observations_by_type(&self, observation_type: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM observation
            WHERE code = $1 AND status != 'deleted'
            "#
        )
        .bind(observation_type)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }
}
