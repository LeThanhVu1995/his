use app_web::prelude::AuthUser;
use app_error::AppError;

// Realm roles
pub const ADMIN: &str = "ADMIN";

// Permissions (scopes) constants
pub const IAM_USER_READ: &str = "iam.user.read";
pub const IAM_USER_CREATE: &str = "iam.user.create";
pub const IAM_USER_UPDATE: &str = "iam.user.update";
pub const IAM_USER_LOCK: &str  = "iam.user.lock";

pub const IAM_ROLE_READ: &str = "iam.role.read";
pub const IAM_ROLE_ASSIGN: &str = "iam.role.assign";

/// Require user to have any of `roles` OR any of `scopes`.
pub fn require(user: &AuthUser, roles: &[&str], scopes: &[&str]) -> Result<(), AppError> {
    let has_role = user.roles.iter().any(|r| roles.iter().any(|x| x == &r.as_str()));
    let has_scope = user.scopes.iter().any(|s| scopes.iter().any(|x| x == &s.as_str()));
    if has_role || has_scope { Ok(()) } else {
        Err(AppError::Forbidden)
    }
}


