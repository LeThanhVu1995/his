use uuid::Uuid;
use sqlx::{Pool, Postgres, Row};

pub struct SagaLogStore<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> SagaLogStore<'a> {
    pub async fn log(
        &self,
        instance_id: Uuid,
        step_id: &str,
        action: &str,
        request: Option<&serde_json::Value>,
        response: Option<&serde_json::Value>,
    ) -> anyhow::Result<()> {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO wf_saga_log(id,instance_id,step_id,action,request,response)
             VALUES($1,$2,$3,$4,$5,$6)"
        )
        .bind(id)
        .bind(instance_id)
        .bind(step_id)
        .bind(action)
        .bind(request)
        .bind(response)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_compensation_steps(
        &self,
        instance_id: Uuid,
    ) -> anyhow::Result<Vec<(String, serde_json::Value)>> {
        let rows = sqlx::query(
            "SELECT step_id, request FROM wf_saga_log
             WHERE instance_id=$1 AND action='forward'
             ORDER BY created_at DESC"
        )
        .bind(instance_id)
        .fetch_all(self.db)
        .await?;

        Ok(rows.into_iter().filter_map(|r| {
            Some((r.try_get("step_id").ok()?, r.try_get("request").ok()?))
        }).collect())
    }
}
