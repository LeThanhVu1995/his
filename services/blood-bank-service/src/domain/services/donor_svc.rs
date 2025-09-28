use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use crate::domain::entities::donor::BloodDonor;
use crate::infra::db::repositories::donor_repo::DonorRepo;

pub struct DonorService<'a> {
    pub donor_repo: DonorRepo<'a>,
}

impl<'a> DonorService<'a> {
    pub async fn register_donor(
        &self,
        code: Option<String>,
        name: String,
        date_of_birth: Option<NaiveDate>,
        gender: Option<String>,
        blood_group: Option<String>,
        phone: Option<String>,
    ) -> anyhow::Result<Uuid> {
        let donor_id = Uuid::new_v4();
        let now = Utc::now();

        let donor = BloodDonor {
            donor_id,
            code,
            name,
            date_of_birth,
            gender,
            blood_group,
            phone,
            created_at: now,
            updated_at: now,
        };

        self.donor_repo.create(&donor).await?;
        Ok(donor_id)
    }

    pub async fn get_donor(&self, donor_id: Uuid) -> anyhow::Result<Option<BloodDonor>> {
        self.donor_repo.get_by_id(donor_id).await
    }

    pub async fn get_donor_by_code(&self, code: &str) -> anyhow::Result<Option<BloodDonor>> {
        self.donor_repo.get_by_code(code).await
    }

    pub async fn list_donors(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<BloodDonor>> {
        self.donor_repo.list(limit, offset).await
    }

    pub async fn update_donor_info(
        &self,
        donor_id: Uuid,
        name: Option<String>,
        date_of_birth: Option<chrono::NaiveDate>,
        gender: Option<String>,
        blood_group: Option<String>,
        phone: Option<String>,
    ) -> anyhow::Result<()> {
        // In a real implementation, you'd update the donor record
        // For now, this is a placeholder
        Ok(())
    }
}
