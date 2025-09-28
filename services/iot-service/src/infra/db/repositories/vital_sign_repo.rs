use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::vital_sign::{VitalSignRecord, VitalSignItem};

pub struct VitalSignRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> VitalSignRepo<'a> {
    pub async fn create_record(&self, record: &VitalSignRecord) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vital_sign_record (vs_id, encounter_id, patient_id, device_id, measured_at, recorder_staff_id, note)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&record.vs_id)
        .bind(&record.encounter_id)
        .bind(&record.patient_id)
        .bind(&record.device_id)
        .bind(&record.measured_at)
        .bind(&record.recorder_staff_id)
        .bind(&record.note)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn create_item(&self, item: &VitalSignItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vital_sign_item (vs_item_id, vs_id, code, value_num, value_text, unit)
            VALUES ($1, $2, $3, $4, $5, $6)
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

    pub async fn get_by_id(&self, vs_id: Uuid) -> anyhow::Result<Option<VitalSignRecord>> {
        let record = sqlx::query_as::<_, VitalSignRecord>(
            r#"
            SELECT vs_id, encounter_id, patient_id, device_id, measured_at, recorder_staff_id, note
            FROM vital_sign_record
            WHERE vs_id = $1
            "#
        )
        .bind(vs_id)
        .fetch_optional(self.db)
        .await?;
        Ok(record)
    }

    pub async fn get_items_by_vs_id(&self, vs_id: Uuid) -> anyhow::Result<Vec<VitalSignItem>> {
        let items = sqlx::query_as::<_, VitalSignItem>(
            r#"
            SELECT vs_item_id, vs_id, code, value_num, value_text, unit
            FROM vital_sign_item
            WHERE vs_id = $1
            ORDER BY code
            "#
        )
        .bind(vs_id)
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn list_by_patient(&self, patient_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<VitalSignRecord>> {
        let records = sqlx::query_as::<_, VitalSignRecord>(
            r#"
            SELECT vs_id, encounter_id, patient_id, device_id, measured_at, recorder_staff_id, note
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

    pub async fn list_by_encounter(&self, encounter_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<VitalSignRecord>> {
        let records = sqlx::query_as::<_, VitalSignRecord>(
            r#"
            SELECT vs_id, encounter_id, patient_id, device_id, measured_at, recorder_staff_id, note
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

    pub async fn count_by_patient(&self, patient_id: Uuid) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM vital_sign_record WHERE patient_id = $1"
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }
}
