use uuid::Uuid;
use sqlx::{Pool, Postgres, Row};
use serde_json::Value as Json;

pub struct TaskStore<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> TaskStore<'a> {
    pub async fn create(
        &self,
        ins: Uuid,
        step_id: &str,
        name: &str,
        candidates: Json,
        payload: Json,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO wf_tasks(id,instance_id,step_id,name,candidate_roles,payload,status)
             VALUES($1,$2,$3,$4,$5,$6,'READY')"
        )
        .bind(id)
        .bind(ins)
        .bind(step_id)
        .bind(name)
        .bind(candidates.as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>()))
        .bind(payload)
        .execute(self.db)
        .await?;
        Ok(id)
    }

    pub async fn claim(&self, id: Uuid, user: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE wf_tasks SET assignee=$2, status='CLAIMED' WHERE id=$1 AND status='READY'"
        )
        .bind(id)
        .bind(user)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn complete(&self, id: Uuid, output: &Json) -> anyhow::Result<(Uuid, String)> {
        let rec = sqlx::query(
            "UPDATE wf_tasks SET status='COMPLETED', completed_at=NOW() WHERE id=$1 RETURNING instance_id, step_id"
        )
        .bind(id)
        .fetch_one(self.db)
        .await?;
        Ok((rec.try_get("instance_id")?, rec.try_get("step_id")?))
    }

    pub async fn get(&self, id: Uuid) -> anyhow::Result<Option<crate::domain::entities::task::Task>> {
        Ok(sqlx::query_as::<_, crate::domain::entities::task::Task>(
            r#"SELECT id,instance_id,step_id,name,assignee,candidate_roles,payload,status,created_at,completed_at
               FROM wf_tasks WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}
