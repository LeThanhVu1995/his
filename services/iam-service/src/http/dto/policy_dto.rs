use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PolicyCreateReq {
    pub code: String,
    pub description: Option<String>,
    pub effect: String,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
    pub condition: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PolicyDto {
    pub id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub effect: String,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
    pub condition: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignPolicyToRoleReq { pub role_id: Uuid, pub policy_id: Uuid }
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignPolicyToUserReq { pub user_id: Uuid, pub policy_id: Uuid }

// Placeholder for future policy DTOs (RBAC/ABAC rules)
