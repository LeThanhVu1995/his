use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::blood_unit::BloodUnit;

#[derive(Clone)]
pub struct BloodUnitRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> BloodUnitRepo<'a> {
    pub async fn create(&self, unit: &BloodUnit) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bb_blood_unit(unit_id, donation_id, component_code, unit_no, blood_group, expiry_date, status, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(unit.unit_id)
        .bind(unit.donation_id)
        .bind(&unit.component_code)
        .bind(&unit.unit_no)
        .bind(&unit.blood_group)
        .bind(unit.expiry_date)
        .bind(&unit.status)
        .bind(unit.created_at)
        .bind(unit.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, unit_id: Uuid) -> anyhow::Result<Option<BloodUnit>> {
        Ok(sqlx::query_as::<_, BloodUnit>(
            "SELECT unit_id, donation_id, component_code, unit_no, blood_group, expiry_date, status, created_at, updated_at
             FROM bb_blood_unit WHERE unit_id = $1"
        )
        .bind(unit_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn get_by_unit_no(&self, unit_no: &str) -> anyhow::Result<Option<BloodUnit>> {
        Ok(sqlx::query_as::<_, BloodUnit>(
            "SELECT unit_id, donation_id, component_code, unit_no, blood_group, expiry_date, status, created_at, updated_at
             FROM bb_blood_unit WHERE unit_no = $1"
        )
        .bind(unit_no)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_available(&self, blood_group: Option<&str>, component_code: Option<&str>) -> anyhow::Result<Vec<BloodUnit>> {
        Ok(sqlx::query_as::<_, BloodUnit>(
            "SELECT unit_id, donation_id, component_code, unit_no, blood_group, expiry_date, status, created_at, updated_at
             FROM bb_blood_unit WHERE status = 'AVAILABLE' AND expiry_date > CURRENT_DATE
             AND ($1::text IS NULL OR blood_group = $1)
             AND ($2::text IS NULL OR component_code = $2)
             ORDER BY created_at DESC"
        )
        .bind(blood_group)
        .bind(component_code)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn update_status(&self, unit_id: Uuid, status: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE bb_blood_unit SET status = $2, updated_at = NOW() WHERE unit_id = $1")
            .bind(unit_id)
            .bind(status)
            .execute(self.db)
            .await?;
        Ok(())
    }

    pub async fn list_by_donation(&self, donation_id: Uuid) -> anyhow::Result<Vec<BloodUnit>> {
        Ok(sqlx::query_as::<_, BloodUnit>(
            "SELECT unit_id, donation_id, component_code, unit_no, blood_group, expiry_date, status, created_at, updated_at
             FROM bb_blood_unit WHERE donation_id = $1 ORDER BY created_at DESC"
        )
        .bind(donation_id)
        .fetch_all(self.db)
        .await?)
    }
}
