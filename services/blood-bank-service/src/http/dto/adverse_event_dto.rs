use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Report Adverse Event
#[derive(Debug, Deserialize)]
pub struct ReportAdverseEventRequest {
    pub issue_id: Uuid,
    pub type_code: String,
    pub severity_code: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ReportAdverseEventResponse {
    pub event_id: Uuid,
    pub message: String,
}

// List Adverse Events
#[derive(Debug, Deserialize)]
pub struct ListAdverseEventsQuery {
    pub issue_id: Option<Uuid>,
    pub type_code: Option<String>,
    pub severity_code: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub from_date: Option<chrono::NaiveDate>,
    pub to_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct ListAdverseEventsResponse {
    pub events: Vec<crate::domain::entities::adverse_event::BloodAdverseEvent>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

// Get Adverse Event
#[derive(Debug, Serialize)]
pub struct GetAdverseEventResponse {
    pub event: crate::domain::entities::adverse_event::BloodAdverseEvent,
    pub issue: Option<crate::domain::entities::issue::Issue>,
}
