// iam-service src/http/dto/user_dto.rs placeholder
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserCreateReq {
    pub username: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateReq {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub locked: bool,
}
