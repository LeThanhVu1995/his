use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::ComponentsBuilder;
use utoipa::{Modify, OpenApi};

use crate::http::dto::{role_dto::*, user_dto::*, policy_dto::*};
use crate::http::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health::health,
        handlers::me::me,
        handlers::users::list::list,
        handlers::users::create::create,
        handlers::users::get::get,
        handlers::users::update::update,
        handlers::users::lock::lock,
        handlers::roles::list::list,
        handlers::roles::assign::assign,
        handlers::policies::list::list,
        handlers::policies::create::create,
        handlers::policies::assign_role::assign_role,
        handlers::policies::assign_user::assign_user,
        handlers::policies::delete::delete,
        handlers::auth::providers::providers,
        handlers::auth::authorize::authorize,
        handlers::auth::logout::logout
    ),
    components(schemas(UserCreateReq, UserUpdateReq, UserDto, RoleDto, AssignRoleReq, PolicyCreateReq, PolicyDto, AssignPolicyToRoleReq, AssignPolicyToUserReq)),
    tags((name = "iam", description = "Identity & Access Management")),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let bearer = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );
        let comps = openapi.components.take().unwrap_or_default();
        openapi.components = Some(
            ComponentsBuilder::from(comps)
                .security_scheme("bearerAuth", bearer)
                .build(),
        );
        openapi.security = Some(vec![utoipa::openapi::security::SecurityRequirement::new(
            "bearerAuth",
            vec![
                "iam.user.read",
                "iam.user.create",
                "iam.user.update",
                "iam.user.lock",
                "iam.role.read",
                "iam.role.assign",
            ],
        )]);
    }
}
