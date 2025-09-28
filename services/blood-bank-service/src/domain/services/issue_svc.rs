use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::issue::Issue;
use crate::infra::db::repositories::{issue_repo::IssueRepo, blood_unit_repo::BloodUnitRepo, crossmatch_repo::CrossmatchRepo};

pub struct IssueService<'a> {
    pub issue_repo: IssueRepo<'a>,
    pub blood_unit_repo: BloodUnitRepo<'a>,
    pub crossmatch_repo: CrossmatchRepo<'a>,
}

impl<'a> IssueService<'a> {
    pub async fn issue_blood_unit(
        &self,
        unit_id: Uuid,
        encounter_id: Uuid,
        issued_by: Uuid,
    ) -> anyhow::Result<Uuid> {
        // Kiểm tra đơn vị máu có tồn tại và available không
        if let Some(unit) = self.blood_unit_repo.get_by_id(unit_id).await? {
            if unit.status != "CROSSMATCHED" {
                anyhow::bail!("Blood unit must be crossmatched before issuing");
            }
        } else {
            anyhow::bail!("Blood unit not found");
        }

        // Kiểm tra có crossmatch record không
        let crossmatches = self.crossmatch_repo.list_by_patient(encounter_id).await?;
        let has_compatible_crossmatch = crossmatches.iter()
            .any(|c| c.unit_id == unit_id && c.result_code == "COMPATIBLE");

        if !has_compatible_crossmatch {
            anyhow::bail!("No compatible crossmatch found for this unit");
        }

        let issue_id = Uuid::new_v4();
        let now = Utc::now();

        let issue = Issue {
            issue_id,
            unit_id,
            encounter_id,
            issued_at: now,
            issued_by,
            created_at: now,
            updated_at: now,
        };

        self.issue_repo.insert(&issue).await?;

        // Cập nhật trạng thái đơn vị máu thành ISSUED
        self.blood_unit_repo.update_status(unit_id, "ISSUED").await?;

        Ok(issue_id)
    }

    pub async fn get_issue(&self, issue_id: Uuid) -> anyhow::Result<Option<Issue>> {
        // Trong thực tế sẽ implement get_by_id trong repo
        Ok(None) // Placeholder
    }

    pub async fn list_issues_by_encounter(&self, encounter_id: Uuid) -> anyhow::Result<Vec<Issue>> {
        self.issue_repo.list_by_encounter(encounter_id).await
    }

    pub async fn return_blood_unit(&self, unit_id: Uuid, reason: &str) -> anyhow::Result<()> {
        // Kiểm tra đơn vị máu có được issue chưa
        if let Some(unit) = self.blood_unit_repo.get_by_id(unit_id).await? {
            if unit.status != "ISSUED" {
                anyhow::bail!("Blood unit is not issued, cannot return");
            }
        } else {
            anyhow::bail!("Blood unit not found");
        }

        // Cập nhật trạng thái về AVAILABLE (nếu chưa hết hạn)
        self.blood_unit_repo.update_status(unit_id, "AVAILABLE").await?;

        // Log return reason (trong thực tế sẽ lưu vào audit log)
        tracing::info!("Blood unit {} returned. Reason: {}", unit_id, reason);

        Ok(())
    }

    pub async fn get_issue_statistics(&self, encounter_id: Uuid) -> anyhow::Result<serde_json::Value> {
        let issues = self.list_issues_by_encounter(encounter_id).await?;

        Ok(serde_json::json!({
            "total_issued": issues.len(),
            "encounter_id": encounter_id,
            "issues": issues
        }))
    }

    pub async fn validate_issue_eligibility(&self, unit_id: Uuid, encounter_id: Uuid) -> anyhow::Result<bool> {
        // Kiểm tra đơn vị máu có available không
        if let Some(unit) = self.blood_unit_repo.get_by_id(unit_id).await? {
            if unit.status != "CROSSMATCHED" {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        // Kiểm tra có crossmatch compatible không
        let crossmatches = self.crossmatch_repo.list_by_patient(encounter_id).await?;
        let has_compatible = crossmatches.iter()
            .any(|c| c.unit_id == unit_id && c.result_code == "COMPATIBLE");

        Ok(has_compatible)
    }
}
