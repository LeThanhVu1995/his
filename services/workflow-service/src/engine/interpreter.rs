use serde_json::Value as Json;
use anyhow::Context;
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::store::{instances::InstanceStore, templates::TemplateStore, tasks::TaskStore, saga_log::SagaLogStore};
use crate::adapters::{http_client, kafka, rule_engine};
use crate::engine::compensation::CompensationEngine;

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
                let step_id = step.get("id").and_then(|v| v.as_str()).unwrap_or("http");

                // Log forward action for saga
                SagaLogStore { db: self.db }.log(
                    id,
                    step_id,
                    "forward",
                    body,
                    None,
                ).await?;

                // Execute HTTP call with error handling
                match http_client::call(method, url, body).await {
                    Ok(resp) => {
                        // Log successful response
                        SagaLogStore { db: self.db }.log(
                            id,
                            step_id,
                            "response",
                            None,
                            Some(&resp),
                        ).await?;

                        if let Some(save) = step.get("save_as").and_then(|v| v.as_str()) {
                            ctx["ctx"][save] = resp;
                        }
                    },
                    Err(e) => {
                        // Log error and handle compensation
                        SagaLogStore { db: self.db }.log(
                            id,
                            step_id,
                            "error",
                            None,
                            Some(&serde_json::json!({"error": e.to_string()})),
                        ).await?;

                        // Check if compensation is needed
                        if step.get("compensate").is_some() {
                            tracing::warn!("HTTP step failed, triggering compensation: {}", e);
                            let compensation_engine = CompensationEngine { db: self.db };
                            compensation_engine.compensate(id).await?;
                        }

                        // Update instance status to failed
                        store.save_progress(id, &ins.cursor, &ctx, "FAILED", None, Some(&e.to_string())).await?;
                        return Err(e);
                    }
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
                let step_id = step.get("id").and_then(|v| v.as_str()).unwrap_or("kafka");

                // Log forward action for saga
                SagaLogStore { db: self.db }.log(
                    id,
                    step_id,
                    "forward",
                    Some(&payload),
                    None,
                ).await?;

                // Execute Kafka publish with error handling
                match kafka::publish(topic, key, &payload).await {
                    Ok(_) => {
                        // Log successful publish
                        SagaLogStore { db: self.db }.log(
                            id,
                            step_id,
                            "published",
                            None,
                            Some(&serde_json::json!({"topic": topic, "key": key})),
                        ).await?;
                    },
                    Err(e) => {
                        // Log error
                        SagaLogStore { db: self.db }.log(
                            id,
                            step_id,
                            "error",
                            None,
                            Some(&serde_json::json!({"error": e.to_string()})),
                        ).await?;

                        // Update instance status to failed
                        store.save_progress(id, &ins.cursor, &ctx, "FAILED", None, Some(&e.to_string())).await?;
                        return Err(e);
                    }
                }
            }

            // Handle parallel execution step
            if let Some(parallel) = step.get("parallel") {
                let branches = parallel.get("branches").and_then(|v| v.as_array()).context("branches")?;

                // Execute all branches in parallel
                let mut handles = Vec::new();
                for (i, branch) in branches.iter().enumerate() {
                    let branch_ctx = ctx.clone();
                    let instance_id = id;
                    let db = self.db;

                    let handle = tokio::spawn(async move {
                        let mut local_ctx = branch_ctx;
                        if let Some(branch_steps) = branch.get("steps").and_then(|v| v.as_array()) {
                            for branch_step in branch_steps {
                                let interpreter = Interpreter { db };
                                if let Err(e) = interpreter.execute_step(branch_step, &mut local_ctx, instance_id).await {
                                    tracing::error!("Branch {} failed: {}", i, e);
                                    return Err(e);
                                }
                            }
                        }
                        Ok(local_ctx)
                    });
                    handles.push(handle);
                }

                // Wait for all branches to complete
                let mut results = Vec::new();
                for handle in handles {
                    match handle.await {
                        Ok(Ok(branch_ctx)) => results.push(branch_ctx),
                        Ok(Err(e)) => return Err(e),
                        Err(e) => return Err(anyhow::anyhow!("Branch execution failed: {}", e)),
                    }
                }

                // Merge results back into main context
                for (i, branch_ctx) in results.into_iter().enumerate() {
                    if let Some(merge_key) = parallel.get("merge_key").and_then(|v| v.as_str()) {
                        ctx["ctx"][format!("{}_{}", merge_key, i)] = branch_ctx["ctx"].clone();
                    }
                }
            }

            // Handle sub-process step
            if let Some(subprocess) = step.get("subprocess") {
                let template_code = subprocess.get("template").and_then(|v| v.as_str()).context("template")?;
                let input = subprocess.get("input").cloned().unwrap_or(Json::Null);

                // Create new instance for sub-process
                let sub_instance_id = store.create(template_code, &input).await?;

                // Execute sub-process
                let sub_interpreter = Interpreter { db: self.db };
                sub_interpreter.tick(sub_instance_id).await?;

                // Get sub-process result
                if let Some(sub_instance) = store.get(sub_instance_id).await? {
                    if let Some(result_key) = subprocess.get("save_as").and_then(|v| v.as_str()) {
                        ctx["ctx"][result_key] = sub_instance.context;
                    }
                }
            }

            // Handle event trigger step
            if let Some(event_trigger) = step.get("event-trigger") {
                let event_name = event_trigger.get("event").and_then(|v| v.as_str()).context("event")?;
                let payload = event_trigger.get("payload").cloned().unwrap_or(Json::Null);

                // Publish event to Kafka
                kafka::publish("workflow-events", event_name, &payload).await?;

                // Wait for event response (if configured)
                if let Some(wait_for) = event_trigger.get("wait_for") {
                    let timeout_secs = wait_for.get("timeout").and_then(|v| v.as_i64()).unwrap_or(30);
                    let response_key = wait_for.get("save_as").and_then(|v| v.as_str()).unwrap_or("event_response");

                    // Set up waiting state
                    let wake_time = Utc::now() + Duration::seconds(timeout_secs);
                    let cursor = serde_json::json!({
                        "step": step_idx,
                        "waiting_for_event": event_name,
                        "response_key": response_key
                    });
                    store.save_progress(id, &cursor, &ctx, "WAITING", Some(wake_time), None).await?;
                    return Ok(());
                }
            }

            // Handle switch/choice step
            if let Some(switch) = step.get("switch") {
                let condition = switch.get("condition").and_then(|v| v.as_str()).context("condition")?;
                let cases = switch.get("cases").and_then(|v| v.as_array()).context("cases")?;

                let result = rule_engine::evaluate_condition(condition, &ctx).await?;
                if result {
                    // Execute first case
                    if let Some(case) = cases.first() {
                        if let Some(case_condition) = case.get("condition") {
                            let case_result = rule_engine::evaluate_condition(
                                case_condition.as_str().unwrap_or("true"),
                                &ctx
                            ).await?;
                            if case_result {
                                // Execute case steps
                                if let Some(case_steps) = case.get("steps").and_then(|v| v.as_array()) {
                                    for case_step in case_steps {
                                        // Recursively execute case step
                                        self.execute_step(case_step, &mut ctx, id).await?;
                                    }
                                }
                            }
                        }
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

    async fn execute_step(&self, step: &Json, ctx: &mut Json, instance_id: Uuid) -> anyhow::Result<()> {
        let store = InstanceStore { db: self.db };

        // Handle HTTP step
        if let Some(http) = step.get("http") {
            let method = http.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
            let url = http.get("url").and_then(|v| v.as_str()).context("url")?;
            let body = http.get("body");
            let step_id = step.get("id").and_then(|v| v.as_str()).unwrap_or("http");

            // Log forward action for saga
            SagaLogStore { db: self.db }.log(
                instance_id,
                step_id,
                "forward",
                body,
                None,
            ).await?;

            // Execute HTTP call with error handling
            match http_client::call(method, url, body).await {
                Ok(resp) => {
                    // Log successful response
                    SagaLogStore { db: self.db }.log(
                        instance_id,
                        step_id,
                        "response",
                        None,
                        Some(&resp),
                    ).await?;

                    if let Some(save) = step.get("save_as").and_then(|v| v.as_str()) {
                        ctx["ctx"][save] = resp;
                    }
                },
                Err(e) => {
                    // Log error and handle compensation
                    SagaLogStore { db: self.db }.log(
                        instance_id,
                        step_id,
                        "error",
                        None,
                        Some(&serde_json::json!({"error": e.to_string()})),
                    ).await?;

                    // Check if compensation is needed
                    if step.get("compensate").is_some() {
                        tracing::warn!("HTTP step failed, triggering compensation: {}", e);
                        let compensation_engine = CompensationEngine { db: self.db };
                        compensation_engine.compensate(instance_id).await?;
                    }

                    return Err(e);
                }
            }
        }

        // Handle Kafka publish step
        if let Some(kafka) = step.get("kafka-publish") {
            let topic = kafka.get("topic").and_then(|v| v.as_str()).context("topic")?;
            let key = kafka.get("key").and_then(|v| v.as_str()).unwrap_or("");
            let payload = kafka.get("payload").cloned().unwrap_or(Json::Null);
            let step_id = step.get("id").and_then(|v| v.as_str()).unwrap_or("kafka");

            // Log forward action for saga
            SagaLogStore { db: self.db }.log(
                instance_id,
                step_id,
                "forward",
                Some(&payload),
                None,
            ).await?;

            // Execute Kafka publish with error handling
            match kafka::publish(topic, key, &payload).await {
                Ok(_) => {
                    // Log successful publish
                    SagaLogStore { db: self.db }.log(
                        instance_id,
                        step_id,
                        "published",
                        None,
                        Some(&serde_json::json!({"topic": topic, "key": key})),
                    ).await?;
                },
                Err(e) => {
                    // Log error
                    SagaLogStore { db: self.db }.log(
                        instance_id,
                        step_id,
                        "error",
                        None,
                        Some(&serde_json::json!({"error": e.to_string()})),
                    ).await?;

                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
