use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::donor::BloodDonor;

pub struct DonorRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DonorRepo<'a> {
    pub async fn create(&self, donor: &BloodDonor) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bb_donor(donor_id, code, name, date_of_birth, gender, blood_group, phone, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(donor.donor_id)
        .bind(&donor.code)
        .bind(&donor.name)
        .bind(donor.date_of_birth)
        .bind(&donor.gender)
        .bind(&donor.blood_group)
        .bind(&donor.phone)
        .bind(donor.created_at)
        .bind(donor.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, donor_id: Uuid) -> anyhow::Result<Option<BloodDonor>> {
        Ok(sqlx::query_as::<_, BloodDonor>(
            "SELECT donor_id, code, name, date_of_birth, gender, blood_group, phone, created_at, updated_at
             FROM bb_donor WHERE donor_id = $1"
        )
        .bind(donor_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn get_by_code(&self, code: &str) -> anyhow::Result<Option<BloodDonor>> {
        Ok(sqlx::query_as::<_, BloodDonor>(
            "SELECT donor_id, code, name, date_of_birth, gender, blood_group, phone, created_at, updated_at
             FROM bb_donor WHERE code = $1"
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<BloodDonor>> {
        Ok(sqlx::query_as::<_, BloodDonor>(
            "SELECT donor_id, code, name, date_of_birth, gender, blood_group, phone, created_at, updated_at
             FROM bb_donor ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }
}
