use uuid::Uuid;
use crate::store::{instances::InstanceStore, saga_log::SagaLogStore};
use crate::adapters::http_client;

pub struct CompensationEngine<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> CompensationEngine<'a> {
    pub async fn compensate(&self, instance_id: Uuid) -> anyhow::Result<()> {
        let saga_store = SagaLogStore { db: self.db };
        let compensation_steps = saga_store.get_compensation_steps(instance_id).await?;

        for (step_id, request) in compensation_steps {
            // Execute compensation step
            if let Some(compensate) = request.get("compensate") {
                if let Some(http) = compensate.get("http") {
                    let method = http.get("method").and_then(|v| v.as_str()).unwrap_or("POST");
                    let url = http.get("url").and_then(|v| v.as_str()).unwrap_or("");
                    let body = http.get("body");

                    // Log compensation action
                    saga_store.log(
                        instance_id,
                        &step_id,
                        "compensate",
                        body,
                        None,
                    ).await?;

                    // Execute compensation HTTP call
                    let _ = http_client::call(method, url, body).await;
                }
            }
        }

        Ok(())
    }
}
