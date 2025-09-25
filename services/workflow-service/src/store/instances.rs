use uuid::Uuid;
use sqlx::{Pool, Postgres, Row};
use chrono::{DateTime, Utc};

pub struct InstanceStore<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> InstanceStore<'a> {
    pub async fn create(
        &self,
        code: &str,
        input: &serde_json::Value,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let ctx = serde_json::json!({"vars": {}, "ctx": {}});
        sqlx::query(
            "INSERT INTO wf_instances(id,template_code,status,input,context,cursor)
             VALUES($1,$2,'PENDING',$3,$4,$5)"
        )
        .bind(id)
        .bind(code)
        .bind(input)
        .bind(ctx)
        .bind(serde_json::json!({"step": 0}))
        .execute(self.db)
        .await?;
        Ok(id)
    }

    pub async fn get(&self, id: Uuid) -> anyhow::Result<Option<crate::domain::entities::instance::Instance>> {
        Ok(sqlx::query_as::<_, crate::domain::entities::instance::Instance>(
            r#"SELECT id,template_code,status,input,context,cursor,error,next_wake_at,created_at,updated_at
               FROM wf_instances WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn save_progress(
        &self,
        id: Uuid,
        cursor: &serde_json::Value,
        context: &serde_json::Value,
        status: &str,
        next: Option<DateTime<Utc>>,
        err: Option<&str>,
    ) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE wf_instances SET cursor=$2, context=$3, status=$4, next_wake_at=$5, error=$6, updated_at=NOW() WHERE id=$1"
        )
        .bind(id)
        .bind(cursor)
        .bind(context)
        .bind(status)
        .bind(next)
        .bind(err)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_waiting(&self) -> anyhow::Result<Vec<Uuid>> {
        let rows = sqlx::query(
            "SELECT id FROM wf_instances WHERE status='WAITING' AND next_wake_at IS NOT NULL AND next_wake_at <= NOW() LIMIT 50"
        )
        .fetch_all(self.db)
        .await?;
        Ok(rows.into_iter().filter_map(|r| r.try_get::<Uuid, _>("id").ok()).collect())
    }
}
