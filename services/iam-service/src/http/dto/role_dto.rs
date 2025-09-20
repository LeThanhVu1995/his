// iam-service src/http/dto/role_dto.rs placeholder
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoleDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignRoleReq {
    pub user_id: Uuid,
    pub role_id: Uuid,
}
