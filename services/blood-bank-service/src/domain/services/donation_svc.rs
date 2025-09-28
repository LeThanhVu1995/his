use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::donation::BloodDonation;
use crate::infra::db::repositories::{donation_repo::DonationRepo, blood_unit_repo::BloodUnitRepo};

pub struct DonationService<'a> {
    pub donation_repo: DonationRepo<'a>,
    pub blood_unit_repo: BloodUnitRepo<'a>,
}

impl<'a> DonationService<'a> {
    pub async fn record_donation(
        &self,
        donor_id: Uuid,
        volume_ml: Option<i32>,
        remarks: Option<String>,
    ) -> anyhow::Result<Uuid> {
        let donation_id = Uuid::new_v4();
        let now = Utc::now();

        let donation = BloodDonation {
            donation_id,
            donor_id,
            collected_at: now,
            volume_ml,
            remarks,
            created_at: now,
            updated_at: now,
        };

        self.donation_repo.create(&donation).await?;
        Ok(donation_id)
    }

    pub async fn get_donation(&self, donation_id: Uuid) -> anyhow::Result<Option<BloodDonation>> {
        self.donation_repo.get_by_id(donation_id).await
    }

    pub async fn list_donations_by_donor(&self, donor_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<BloodDonation>> {
        self.donation_repo.list_by_donor(donor_id, limit, offset).await
    }

    pub async fn list_recent_donations(&self, limit: i64) -> anyhow::Result<Vec<BloodDonation>> {
        self.donation_repo.list_recent(limit).await
    }

    pub async fn process_donation_into_units(
        &self,
        donation_id: Uuid,
        component_codes: Vec<String>, // e.g., ["WB", "PRBC", "FFP"]
        blood_group: Option<String>,
    ) -> anyhow::Result<Vec<Uuid>> {
        let mut unit_ids = Vec::new();
        let now = Utc::now();

        for component_code in component_codes {
            let unit_id = Uuid::new_v4();
            let unit_no = format!("{}-{}", component_code, unit_id.to_string()[..8].to_uppercase());

            // Calculate expiry date (typically 35-42 days for blood components)
            let expiry_days = match component_code.as_str() {
                "WB" => 35,
                "PRBC" => 42,
                "FFP" => 365,
                "PLT" => 5,
                _ => 35,
            };
            let expiry_date = (now + chrono::Duration::days(expiry_days)).date_naive();

            let blood_unit = crate::domain::entities::blood_unit::BloodUnit {
                unit_id,
                donation_id,
                component_code: Some(component_code),
                unit_no: Some(unit_no),
                blood_group: blood_group.clone(),
                expiry_date: Some(expiry_date),
                status: "AVAILABLE".to_string(),
                created_at: now,
                updated_at: now,
            };

            self.blood_unit_repo.create(&blood_unit).await?;
            unit_ids.push(unit_id);
        }

        Ok(unit_ids)
    }
}
