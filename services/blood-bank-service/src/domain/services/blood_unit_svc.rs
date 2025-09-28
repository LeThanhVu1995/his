use uuid::Uuid;
use crate::domain::entities::blood_unit::BloodUnit;
use crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo;

pub struct BloodUnitService<'a> {
    pub blood_unit_repo: BloodUnitRepo<'a>,
}

impl<'a> BloodUnitService<'a> {
    pub async fn get_available_units(
        &self,
        blood_group: Option<&str>,
        component_code: Option<&str>,
    ) -> anyhow::Result<Vec<BloodUnit>> {
        self.blood_unit_repo.list_available(blood_group, component_code).await
    }

    pub async fn get_unit(&self, unit_id: Uuid) -> anyhow::Result<Option<BloodUnit>> {
        self.blood_unit_repo.get_by_id(unit_id).await
    }

    pub async fn get_unit_by_number(&self, unit_no: &str) -> anyhow::Result<Option<BloodUnit>> {
        self.blood_unit_repo.get_by_unit_no(unit_no).await
    }

    pub async fn reserve_unit(&self, unit_id: Uuid) -> anyhow::Result<()> {
        self.blood_unit_repo.update_status(unit_id, "RESERVED").await
    }

    pub async fn issue_unit(&self, unit_id: Uuid) -> anyhow::Result<()> {
        self.blood_unit_repo.update_status(unit_id, "ISSUED").await
    }

    pub async fn return_unit(&self, unit_id: Uuid) -> anyhow::Result<()> {
        self.blood_unit_repo.update_status(unit_id, "AVAILABLE").await
    }

    pub async fn update_status(&self, unit_id: Uuid, status: &str) -> anyhow::Result<()> {
        self.blood_unit_repo.update_status(unit_id, status).await
    }

    pub async fn expire_unit(&self, unit_id: Uuid) -> anyhow::Result<()> {
        self.blood_unit_repo.update_status(unit_id, "EXPIRED").await
    }

    pub async fn check_compatibility(
        &self,
        patient_blood_group: &str,
        unit_blood_group: &str,
    ) -> bool {
        // Basic ABO compatibility rules
        match (patient_blood_group, unit_blood_group) {
            // Universal recipient
            ("AB+", _) => true,
            ("AB-", "A-" | "B-" | "AB-" | "O-") => true,

            // A group
            ("A+", "A+" | "A-" | "O+" | "O-") => true,
            ("A-", "A-" | "O-") => true,

            // B group
            ("B+", "B+" | "B-" | "O+" | "O-") => true,
            ("B-", "B-" | "O-") => true,

            // O group
            ("O+", "O+" | "O-") => true,
            ("O-", "O-") => true,

            _ => false,
        }
    }

    pub async fn find_compatible_units(
        &self,
        patient_blood_group: &str,
        component_code: Option<&str>,
    ) -> anyhow::Result<Vec<BloodUnit>> {
        let available_units = self.get_available_units(Some(patient_blood_group), component_code).await?;

        let mut compatible_units = Vec::new();
        for unit in available_units {
            if let Some(unit_bg) = &unit.blood_group {
                if self.check_compatibility(patient_blood_group, unit_bg).await {
                    compatible_units.push(unit);
                }
            }
        }

        Ok(compatible_units)
    }
}
