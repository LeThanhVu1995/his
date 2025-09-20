use actix_web::{FromRequest, HttpRequest, HttpMessage};
use futures_util::future::{ready, Ready};

use app_error::AppError;

/// Thông tin user tối thiểu do AuthMiddleware đính kèm sau khi validate token.
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: String,
    pub subject: String,
    pub preferred_username: Option<String>,
    pub roles: Vec<String>,
    pub scopes: Vec<String>,
    pub tenant_id: Option<String>,
}

impl AuthUser {
    pub fn has_role(&self, r: &str) -> bool {
        self.roles.iter().any(|x| x == r)
    }
    pub fn has_scope(&self, s: &str) -> bool {
        self.scopes.iter().any(|x| x == s)
    }
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(u) = req.extensions().get::<AuthUser>() {
            return ready(Ok(u.clone()));
        }
        ready(Err(AppError::Unauthorized.into()))
    }
}

/// Biến thể Optional: không có user thì trả None.
#[derive(Clone, Debug)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

impl FromRequest for MaybeAuthUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let u = req.extensions().get::<AuthUser>().cloned();
        ready(Ok(MaybeAuthUser(u)))
    }
}
