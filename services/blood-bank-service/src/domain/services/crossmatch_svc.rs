use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::crossmatch::Crossmatch;
use crate::infra::db::repositories::{crossmatch_repo::CrossmatchRepo, blood_unit_repo::BloodUnitRepo};

pub struct CrossmatchService<'a> {
    pub crossmatch_repo: CrossmatchRepo<'a>,
    pub blood_unit_repo: BloodUnitRepo<'a>,
}

impl<'a> CrossmatchService<'a> {
    pub async fn perform_crossmatch(
        &self,
        patient_id: Uuid,
        unit_id: Uuid,
        performer_id: Uuid,
    ) -> anyhow::Result<Uuid> {
        // Kiểm tra đơn vị máu có tồn tại và available không
        if let Some(unit) = self.blood_unit_repo.get_by_id(unit_id).await? {
            if unit.status != "AVAILABLE" {
                anyhow::bail!("Blood unit is not available for crossmatch");
            }
        } else {
            anyhow::bail!("Blood unit not found");
        }

        let crossmatch_id = Uuid::new_v4();
        let now = Utc::now();

        // Thực hiện crossmatch logic (simplified)
        let result_code = self.determine_compatibility(patient_id, unit_id).await?;

        let crossmatch = Crossmatch {
            crossmatch_id,
            patient_id,
            unit_id,
            performed_at: now,
            result_code: result_code.clone(),
            performer_id,
            created_at: now,
            updated_at: now,
        };

        self.crossmatch_repo.insert(&crossmatch).await?;

        // Cập nhật trạng thái đơn vị máu nếu compatible
        if result_code == "COMPATIBLE" {
            self.blood_unit_repo.update_status(unit_id, "CROSSMATCHED").await?;
        }

        Ok(crossmatch_id)
    }

    async fn determine_compatibility(&self, patient_id: Uuid, unit_id: Uuid) -> anyhow::Result<String> {
        // Trong thực tế, đây sẽ là logic phức tạp:
        // 1. Lấy thông tin nhóm máu bệnh nhân từ EMR
        // 2. Lấy thông tin nhóm máu đơn vị máu
        // 3. Kiểm tra ABO compatibility
        // 4. Kiểm tra Rh compatibility
        // 5. Kiểm tra kháng thể bất thường
        // 6. Thực hiện major/minor crossmatch

        // Simplified logic - trong thực tế sẽ gọi EMR service
        let unit = self.blood_unit_repo.get_by_id(unit_id).await?
            .ok_or_else(|| anyhow::anyhow!("Unit not found"))?;

        // Mock compatibility check - trong thực tế sẽ có logic phức tạp
        match unit.blood_group.as_deref() {
            Some("O+") | Some("O-") => Ok("COMPATIBLE".to_string()),
            Some("A+") | Some("A-") => Ok("COMPATIBLE".to_string()),
            Some("B+") | Some("B-") => Ok("COMPATIBLE".to_string()),
            Some("AB+") | Some("AB-") => Ok("COMPATIBLE".to_string()),
            _ => Ok("INCOMPATIBLE".to_string()),
        }
    }

    pub async fn get_crossmatch(&self, crossmatch_id: Uuid) -> anyhow::Result<Option<Crossmatch>> {
        // Trong thực tế sẽ implement get_by_id trong repo
        Ok(None) // Placeholder
    }

    pub async fn list_crossmatches_by_patient(&self, patient_id: Uuid) -> anyhow::Result<Vec<Crossmatch>> {
        self.crossmatch_repo.list_by_patient(patient_id).await
    }

    pub async fn get_compatible_units_for_patient(
        &self,
        patient_id: Uuid,
        blood_group: &str,
        component_code: Option<&str>,
    ) -> anyhow::Result<Vec<Uuid>> {
        // Lấy danh sách đơn vị máu available
        let available_units = self.blood_unit_repo.list_available(Some(blood_group), component_code).await?;

        let mut compatible_units = Vec::new();

        for unit in available_units {
            // Thực hiện crossmatch cho từng unit
            let result = self.determine_compatibility(patient_id, unit.unit_id).await?;
            if result == "COMPATIBLE" {
                compatible_units.push(unit.unit_id);
            }
        }

        Ok(compatible_units)
    }

    pub async fn validate_crossmatch_required(&self, patient_id: Uuid) -> bool {
        // Trong thực tế sẽ kiểm tra:
        // 1. Bệnh nhân có kháng thể bất thường không
        // 2. Có tiền sử phản ứng truyền máu không
        // 3. Loại phẫu thuật có yêu cầu crossmatch không

        // Simplified - luôn yêu cầu crossmatch
        true
    }
}
