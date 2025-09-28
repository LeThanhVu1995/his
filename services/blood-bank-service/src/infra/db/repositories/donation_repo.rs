use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::donation::BloodDonation;

pub struct DonationRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DonationRepo<'a> {
    pub async fn create(&self, donation: &BloodDonation) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bb_donation(donation_id, donor_id, collected_at, volume_ml, remarks, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(donation.donation_id)
        .bind(donation.donor_id)
        .bind(donation.collected_at)
        .bind(donation.volume_ml)
        .bind(&donation.remarks)
        .bind(donation.created_at)
        .bind(donation.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, donation_id: Uuid) -> anyhow::Result<Option<BloodDonation>> {
        Ok(sqlx::query_as::<_, BloodDonation>(
            "SELECT donation_id, donor_id, collected_at, volume_ml, remarks, created_at, updated_at
             FROM bb_donation WHERE donation_id = $1"
        )
        .bind(donation_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_donor(&self, donor_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<BloodDonation>> {
        Ok(sqlx::query_as::<_, BloodDonation>(
            "SELECT donation_id, donor_id, collected_at, volume_ml, remarks, created_at, updated_at
             FROM bb_donation WHERE donor_id = $1 ORDER BY collected_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(donor_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<BloodDonation>> {
        Ok(sqlx::query_as::<_, BloodDonation>(
            "SELECT donation_id, donor_id, collected_at, volume_ml, remarks, created_at, updated_at
             FROM bb_donation ORDER BY collected_at DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(self.db)
        .await?)
    }
}
