use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::adverse_event::BloodAdverseEvent;
use crate::infra::db::repositories::adverse_event_repo::AdverseEventRepo;

pub struct AdverseEventService<'a> {
    pub adverse_event_repo: AdverseEventRepo<'a>,
}

impl<'a> AdverseEventService<'a> {
    pub async fn report_adverse_event(
        &self,
        issue_id: Uuid,
        type_code: Option<String>,
        severity_code: Option<String>,
        description: Option<String>,
    ) -> anyhow::Result<Uuid> {
        let event_id = Uuid::new_v4();
        let now = Utc::now();

        let event = BloodAdverseEvent {
            event_id,
            issue_id,
            event_time: now,
            type_code,
            severity_code,
            description,
            created_at: now,
            updated_at: now,
        };

        self.adverse_event_repo.create(&event).await?;
        Ok(event_id)
    }

    pub async fn get_event(&self, event_id: Uuid) -> anyhow::Result<Option<BloodAdverseEvent>> {
        self.adverse_event_repo.get_by_id(event_id).await
    }

    pub async fn list_events_by_issue(&self, issue_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<BloodAdverseEvent>> {
        self.adverse_event_repo.list_by_issue(issue_id, limit, offset).await
    }

    pub async fn list_recent_events(&self, limit: i64) -> anyhow::Result<Vec<BloodAdverseEvent>> {
        self.adverse_event_repo.list_recent(limit).await
    }

    pub async fn classify_severity(&self, description: &str) -> String {
        let description_lower = description.to_lowercase();

        if description_lower.contains("severe") || description_lower.contains("life-threatening") {
            "SEVERE".to_string()
        } else if description_lower.contains("moderate") || description_lower.contains("significant") {
            "MODERATE".to_string()
        } else if description_lower.contains("mild") || description_lower.contains("minor") {
            "MILD".to_string()
        } else {
            "UNKNOWN".to_string()
        }
    }

    pub async fn get_event_statistics(&self) -> anyhow::Result<serde_json::Value> {
        // This would typically involve complex queries
        // For now, return a placeholder
        Ok(serde_json::json!({
            "total_events": 0,
            "by_severity": {
                "SEVERE": 0,
                "MODERATE": 0,
                "MILD": 0,
                "UNKNOWN": 0
            },
            "by_type": {}
        }))
    }
}
