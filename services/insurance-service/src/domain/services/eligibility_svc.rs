use uuid::Uuid;
use crate::infrastructure::repositories::member_repo::MemberRepo;
use crate::domain::entities::member::Member;

pub struct EligibilitySvc<'a> {
    pub repo: MemberRepo<'a>,
}

impl<'a> EligibilitySvc<'a> {
    pub async fn check_and_upsert(&self, patient: Uuid, payer: &str, policy: &str) -> anyhow::Result<Member> {
        // For now, we'll simulate eligibility check
        // In production, this would call external BHYT or private insurer APIs
        let eligible = true; // Mock: always eligible for now
        let plan_code = Some("BHYT-A".to_string());
        let start_date = Some(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        let end_date = Some(chrono::NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());

        if !eligible {
            anyhow::bail!("not eligible");
        }

        let m = Member {
            id: uuid::Uuid::new_v4(),
            patient_id: patient,
            payer: payer.to_string(),
            policy_no: policy.to_string(),
            plan_code,
            start_date,
            end_date,
            status: "ACTIVE".into(),
            holder_name: None,
            note: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.repo.upsert(&m).await?;
        Ok(m)
    }
}
