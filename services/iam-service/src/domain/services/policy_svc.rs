use globset::Glob;
use serde_json::Value;
use crate::infra::db::pool::PgPool;
use crate::infra::db::repositories::policy_repo;
use app_web::prelude::AuthUser;

fn matches_any(text: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|p| Glob::new(p).ok().map(|g| g.compile_matcher().is_match(text)).unwrap_or(false))
}

fn match_resource(resource: Option<&str>, patterns: &[String]) -> bool {
    let r = resource.unwrap_or("*");
    matches_any(r, patterns)
}

fn eval_condition(cond: &Option<Value>, ctx: Option<&Value>) -> bool {
    match cond {
        None => true,
        Some(Value::Null) => true,
        Some(Value::Object(map)) => {
            let ctxm = match ctx { Some(Value::Object(m)) => m, _ => return false };
            for (k, v) in map.iter() {
                if ctxm.get(k) != Some(v) { return false; }
            }
            true
        }
        _ => false,
    }
}

/// Returns Ok(()) if allowed, Forbidden otherwise.
pub async fn is_allowed(db: &PgPool, user: &AuthUser, action: &str, resource: Option<&str>, ctx: Option<&Value>) -> Result<(), app_error::AppError> {
    // Token quick-pass: ADMIN role or scope wildcard match
    let scope_allows = user.scopes.iter().any(|s| Glob::new(s).ok().map(|g| g.compile_matcher().is_match(action)).unwrap_or(false));
    let has_admin = user.roles.iter().any(|r| r == "ADMIN");
    if has_admin || scope_allows { return Ok(()); }

    let policies = policy_repo::policies_for_user(db, &user.user_id, &user.roles).await?;

    let mut allow_hit = false;
    for p in policies.iter() {
        if !matches_any(action, &p.actions) { continue; }
        if !match_resource(resource, &p.resources) { continue; }
        if !eval_condition(&p.condition, ctx) { continue; }
        match p.effect.as_str() { "deny" => return Err(app_error::AppError::Forbidden), _ => allow_hit = true }
    }

    if allow_hit { Ok(()) } else { Err(app_error::AppError::Forbidden) }
}


