use uuid::Uuid;
use sqlx::Row;
use crate::infra::db::pool::PgPool;
use crate::domain::entities::policy::Policy;

pub async fn create(db: &PgPool, req: crate::http::dto::policy_dto::PolicyCreateReq) -> Result<Policy, app_error::AppError> {
    let row = sqlx::query(
        r#"INSERT INTO iam_policies (code, description, effect, actions, resources, condition)
           VALUES ($1,$2,$3,$4,$5,$6)
           RETURNING id, code, description, effect, actions, resources, condition, created_at"#
    )
    .bind(&req.code)
    .bind(&req.description)
    .bind(&req.effect)
    .bind(&req.actions)
    .bind(&req.resources)
    .bind(&req.condition)
    .fetch_one(db)
    .await?;

    Ok(Policy {
        id: row.get("id"),
        code: row.get("code"),
        description: row.get("description"),
        effect: row.get("effect"),
        actions: row.get("actions"),
        resources: row.get("resources"),
        condition: row.get("condition"),
        created_at: row.get("created_at"),
    })
}

pub async fn list(db: &PgPool) -> Result<Vec<Policy>, app_error::AppError> {
    let rows = sqlx::query(
        r#"SELECT id, code, description, effect, actions, resources, condition, created_at
           FROM iam_policies ORDER BY created_at DESC"#
    )
    .fetch_all(db)
    .await?;

    let items: Vec<Policy> = rows.into_iter().map(|row| Policy {
        id: row.get("id"),
        code: row.get("code"),
        description: row.get("description"),
        effect: row.get("effect"),
        actions: row.get("actions"),
        resources: row.get("resources"),
        condition: row.get("condition"),
        created_at: row.get("created_at"),
    }).collect();

    Ok(items)
}

pub async fn delete(db: &PgPool, id: Uuid) -> Result<(), app_error::AppError> {
    sqlx::query("DELETE FROM iam_policies WHERE id=$1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn assign_to_role(db: &PgPool, role_id: Uuid, policy_id: Uuid) -> Result<(), app_error::AppError> {
    sqlx::query(
        r#"INSERT INTO iam_role_policies (role_id, policy_id)
           VALUES ($1,$2) ON CONFLICT DO NOTHING"#
    )
    .bind(role_id)
    .bind(policy_id)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn assign_to_user(db: &PgPool, user_id: Uuid, policy_id: Uuid) -> Result<(), app_error::AppError> {
    sqlx::query(
        r#"INSERT INTO iam_user_policies (user_id, policy_id)
           VALUES ($1,$2) ON CONFLICT DO NOTHING"#
    )
    .bind(user_id)
    .bind(policy_id)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn policies_for_user(db: &PgPool, user_id: &str, roles: &[String]) -> Result<Vec<Policy>, app_error::AppError> {
    let user_rows = sqlx::query(
        r#"SELECT p.id, p.code, p.description, p.effect, p.actions, p.resources, p.condition, p.created_at
           FROM iam_policies p
           JOIN iam_user_policies up ON up.policy_id = p.id
           WHERE up.user_id::text = $1"#
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let role_rows = sqlx::query(
        r#"SELECT p.id, p.code, p.description, p.effect, p.actions, p.resources, p.condition, p.created_at
           FROM iam_policies p
           JOIN iam_role_policies rp ON rp.policy_id = p.id
           JOIN iam_roles r ON r.id = rp.role_id
           WHERE r.code = ANY($1)"#
    )
    .bind(&roles.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    .fetch_all(db)
    .await?;

    let mut items: Vec<Policy> = user_rows.into_iter().map(|row| Policy {
        id: row.get("id"),
        code: row.get("code"),
        description: row.get("description"),
        effect: row.get("effect"),
        actions: row.get("actions"),
        resources: row.get("resources"),
        condition: row.get("condition"),
        created_at: row.get("created_at"),
    }).collect();

    let role_policies: Vec<Policy> = role_rows.into_iter().map(|row| Policy {
        id: row.get("id"),
        code: row.get("code"),
        description: row.get("description"),
        effect: row.get("effect"),
        actions: row.get("actions"),
        resources: row.get("resources"),
        condition: row.get("condition"),
        created_at: row.get("created_at"),
    }).collect();

    items.extend(role_policies);
    Ok(items)
}


