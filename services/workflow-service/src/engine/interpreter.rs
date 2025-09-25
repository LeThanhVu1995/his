use serde_json::Value as Json;
use anyhow::Context;
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::store::{instances::InstanceStore, templates::TemplateStore, tasks::TaskStore, saga_log::SagaLogStore};
use crate::adapters::{http_client, kafka, rule_engine};

pub struct Interpreter<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> Interpreter<'a> {
    pub async fn tick(&self, id: Uuid) -> anyhow::Result<()> {
        let store = InstanceStore { db: self.db };
        let mut ins = store.get(id).await?.context("instance")?;
        let spec: Json = TemplateStore { db: self.db }.get(&ins.template_code).await?.context("template")?;
        let steps = spec.get("steps").and_then(|v| v.as_array()).context("steps")?;
        let mut step_idx = ins.cursor.get("step").and_then(|v| v.as_i64()).unwrap_or(0) as usize;
        let mut ctx = ins.context.clone();

        while step_idx < steps.len() {
            let step = &steps[step_idx];

            // Handle timer step
            if step.get("timer").is_some() {
                let secs = step["timer"].get("seconds").and_then(|v| v.as_i64()).unwrap_or(0);
                let wake = Utc::now() + Duration::seconds(secs);
                let cursor = serde_json::json!({"step": step_idx, "waiting": true});
                store.save_progress(id, &cursor, &ctx, "WAITING", Some(wake), None).await?;
                return Ok(());
            }

            // Handle HTTP step
            if let Some(http) = step.get("http") {
                let method = http.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
                let url = http.get("url").and_then(|v| v.as_str()).context("url")?;
                let body = http.get("body");

                // Log forward action for saga
                SagaLogStore { db: self.db }.log(
                    id,
                    step.get("id").and_then(|v| v.as_str()).unwrap_or("http"),
                    "forward",
                    body,
                    None,
                ).await?;

                let resp = http_client::call(method, url, body).await?;
                if let Some(save) = step.get("save_as").and_then(|v| v.as_str()) {
                    ctx["ctx"][save] = resp;
                }
            }

            // Handle task step
            if let Some(task) = step.get("task") {
                let task_id = TaskStore { db: self.db }.create(
                    id,
                    step.get("id").and_then(|v| v.as_str()).unwrap_or("task"),
                    task.get("name").and_then(|v| v.as_str()).unwrap_or("Task"),
                    task.get("candidate_roles").cloned().unwrap_or(Json::Null),
                    task.get("payload").cloned().unwrap_or(Json::Null),
                ).await?;
                let cursor = serde_json::json!({"step": step_idx, "task_id": task_id});
                store.save_progress(id, &cursor, &ctx, "WAITING", None, None).await?;
                return Ok(());
            }

            // Handle Kafka publish step
            if let Some(kafka) = step.get("kafka-publish") {
                let topic = kafka.get("topic").and_then(|v| v.as_str()).context("topic")?;
                let key = kafka.get("key").and_then(|v| v.as_str()).unwrap_or("");
                let payload = kafka.get("payload").cloned().unwrap_or(Json::Null);

                kafka::publish(topic, key, &payload).await?;
            }

            // Handle switch/choice step
            if let Some(switch) = step.get("switch") {
                let condition = switch.get("condition").and_then(|v| v.as_str()).context("condition")?;
                let cases = switch.get("cases").and_then(|v| v.as_array()).context("cases")?;

                let result = rule_engine::evaluate_condition(condition, &ctx).await?;
                if result {
                    // Execute first case
                    if let Some(case) = cases.first() {
                        // TODO: Execute case steps
                    }
                }
            }

            step_idx += 1;
            ins.cursor = serde_json::json!({"step": step_idx});
            store.save_progress(
                id,
                &ins.cursor,
                &ctx,
                if step_idx < steps.len() { "RUNNING" } else { "COMPLETED" },
                None,
                None,
            ).await?;
        }
        Ok(())
    }
}
