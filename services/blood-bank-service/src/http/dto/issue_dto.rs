use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Issue Blood Unit
#[derive(Debug, Deserialize)]
pub struct IssueBloodRequest {
    pub unit_id: Uuid,
    pub encounter_id: Uuid,
    pub issued_by: Uuid,
}

#[derive(Debug, Serialize)]
pub struct IssueBloodResponse {
    pub issue_id: Uuid,
    pub message: String,
}

// Return Blood Unit
#[derive(Debug, Deserialize)]
pub struct ReturnBloodRequest {
    pub unit_id: Uuid,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct ReturnBloodResponse {
    pub message: String,
}

// List Issues
#[derive(Debug, Deserialize)]
pub struct ListIssuesQuery {
    pub encounter_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub issued_by: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub from_date: Option<chrono::NaiveDate>,
    pub to_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct ListIssuesResponse {
    pub issues: Vec<crate::domain::entities::issue::Issue>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

// Get Issue
#[derive(Debug, Serialize)]
pub struct GetIssueResponse {
    pub issue: crate::domain::entities::issue::Issue,
    pub unit: Option<crate::domain::entities::blood_unit::BloodUnit>,
}
