use serde_json::Value as Json;
use anyhow::Context;
use uuid::Uuid;
use chrono::{Utc, Duration};
use tokio::time::{sleep, timeout};
use crate::store::{instances::InstanceStore, templates::TemplateStore, tasks::TaskStore, saga_log::SagaLogStore};
use crate::adapters::{http_client, kafka, rule_engine, resilience};
use crate::engine::compensation::CompensationEngine;

pub struct Interpreter<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> Interpreter<'a> {
    pub fn tick(&self, id: Uuid) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>> {
        Box::pin(async move {
        let store = InstanceStore { db: self.db };
        let mut ins = store.get(id).await?.context("instance")?;

        // Use locked template version for this instance
        let spec: Json = if let Some(version) = ins.template_version {
            TemplateStore { db: self.db }.get_by_version(&ins.template_code, version).await?.context("template")?
        } else {
            TemplateStore { db: self.db }.get(&ins.template_code).await?.context("template")?
        };
        let steps: Vec<Json> = spec.get("steps").and_then(|v| v.as_array()).context("steps")?.clone();
        let mut step_idx = ins.cursor.get("step").and_then(|v| v.as_i64()).unwrap_or(0) as usize;
        let mut ctx = ins.context.clone();

        while step_idx < steps.len() {
            let step = steps[step_idx].clone();

            // Handle try/catch/finally block
            if let Some(try_block) = step.get("try") {
                let mut error_raised: Option<anyhow::Error> = None;
                if let Some(try_steps) = try_block.get("steps").and_then(|v| v.as_array()) {
                for try_step in try_steps {
                        if let Err(e) = self.execute_step(try_step.clone(), &mut ctx, id).await {
                            error_raised = Some(e);
                            break;
                        }
                    }
                }

                if let Some(err) = error_raised {
                    // Find matching catch
                    if let Some(catches) = step.get("catch").and_then(|v| v.as_array()) {
                        let mut handled = false;
                        for c in catches {
                            let on = c.get("on").and_then(|v| v.as_str()).unwrap_or("any");
                            let matches = match on {
                                "http_error" => true, // simplified mapping
                                "kafka_error" => true,
                                "timeout" => true,
                                "any" => true,
                                _ => false,
                            };
                            if matches {
                                if let Some(steps) = c.get("steps").and_then(|v| v.as_array()) {
                                for s in steps { let _ = self.execute_step(s.clone(), &mut ctx, id).await; }
                                }
                                handled = true;
                                break;
                            }
                        }
                        if !handled {
                            return Err(err);
                        }
                    } else {
                        return Err(err);
                    }
                }

                // finally
                if let Some(finally) = step.get("finally").and_then(|v| v.as_array()) {
                    for s in finally { let _ = self.execute_step(s.clone(), &mut ctx, id).await; }
                }

                // proceed to next step
                step_idx += 1;
                ins.cursor = serde_json::json!({"step": step_idx});
                store.save_progress(
                    id, &ins.cursor, &ctx,
                    if step_idx < steps.len() { "RUNNING" } else { "COMPLETED" },
                    None, None,
                ).await?;
                continue;
            }

            // Handle loop step (while loop)
            if let Some(loop_config) = step.get("loop") {
                let while_condition = loop_config.get("while").and_then(|v| v.as_str()).context("while condition")?;
                let max_iter = loop_config.get("max_iter").and_then(|v| v.as_i64()).unwrap_or(1000);
                let loop_steps = loop_config.get("steps").and_then(|v| v.as_array()).context("loop steps")?;

                // Get current iteration from cursor
                let current_iter = ins.cursor.get("loop_iter").and_then(|v| v.as_i64()).unwrap_or(0);

                if current_iter < max_iter {
                    // Evaluate while condition
                    let should_continue = rule_engine::evaluate_condition(while_condition, &ctx).await?;

                    if should_continue {
                        // Execute loop steps
                        for loop_step in loop_steps {
                            if let Err(e) = self.execute_step(loop_step.clone(), &mut ctx, id).await {
                                tracing::error!("Loop step failed at iteration {}: {}", current_iter, e);
                                return Err(e);
                            }
                        }

                        // Increment iteration and save progress
                        let new_iter = current_iter + 1;
                        let cursor = serde_json::json!({
                            "step": step_idx,
                            "loop_iter": new_iter
                        });
                        store.save_progress(id, &cursor, &ctx, "RUNNING", None, None).await?;

                        // Continue to next iteration
                        continue;
                    }
                }

                // Loop completed, clear loop state and proceed to next step
                let cursor = serde_json::json!({"step": step_idx + 1});
                store.save_progress(id, &cursor, &ctx, "RUNNING", None, None).await?;
                step_idx += 1;
                continue;
            }

            // Handle for-each step
            if let Some(foreach) = step.get("foreach") {
                let items = foreach.get("items").and_then(|v| v.as_array()).context("foreach items")?;
                let as_var = foreach.get("as").and_then(|v| v.as_str()).unwrap_or("item");
                let foreach_steps = foreach.get("steps").and_then(|v| v.as_array()).context("foreach steps")?;

                // Get current index from cursor
                let current_index = ins.cursor.get("foreach_index").and_then(|v| v.as_i64()).unwrap_or(0) as usize;

                if current_index < items.len() {
                    // Set current item in context
                    ctx["vars"][as_var] = items[current_index].clone();

                    // Execute foreach steps
                    for foreach_step in foreach_steps {
                        if let Err(e) = self.execute_step(foreach_step.clone(), &mut ctx, id).await {
                            tracing::error!("Foreach step failed at index {}: {}", current_index, e);
                            return Err(e);
                        }
                    }

                    // Increment index and save progress
                    let new_index = current_index + 1;
                    let cursor = serde_json::json!({
                        "step": step_idx,
                        "foreach_index": new_index
                    });
                    store.save_progress(id, &cursor, &ctx, "RUNNING", None, None).await?;

                    // Continue to next iteration
                    continue;
                }

                // Foreach completed, clear foreach state and proceed to next step
                let cursor = serde_json::json!({"step": step_idx + 1});
                store.save_progress(id, &cursor, &ctx, "RUNNING", None, None).await?;
                step_idx += 1;
                continue;
            }

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

                // Execute HTTP call with timeout + retry policies
                match self.exec_with_policies_http(&step, method, url, body, id, step_id).await {
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

                // Execute Kafka publish with timeout + retry policies
                match self.exec_with_policies_kafka(&step, topic, key, &payload, id, step_id).await {
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

            // Handle dynamic parallel execution (parallel_for)
            if let Some(parallel_for) = step.get("parallel_for") {
                let items = parallel_for.get("items").and_then(|v| v.as_array()).context("items")?;
                let as_var = parallel_for.get("as").and_then(|v| v.as_str()).unwrap_or("item");
                let merge_key = parallel_for.get("merge_key").and_then(|v| v.as_str()).unwrap_or("results");
                let parallel_for_steps: Vec<Json> = parallel_for.get("steps").and_then(|v| v.as_array()).map(|v| v.iter().map(|s| s.clone()).collect()).unwrap_or_default();

                // Execute each item in parallel
                let mut handles = Vec::new();
                for (i, item) in items.iter().enumerate() {
                    let item_ctx = ctx.clone();
                    let instance_id = id;
                    let db = self.db.clone();
                    let item_value = item.clone();
                    let as_var_name = as_var.to_string();
                    let parallel_for_steps_clone = parallel_for_steps.clone();

                    let handle = tokio::spawn(async move {
                        let mut local_ctx = item_ctx;
                        // Set the item variable in context
                        local_ctx["vars"][&as_var_name] = item_value;

                                for step in &parallel_for_steps_clone {
                            let interpreter = Interpreter { db: &db };
                            if let Err(e) = interpreter.execute_step(step.clone(), &mut local_ctx, instance_id).await {
                                tracing::error!("Parallel_for item {} failed: {}", i, e);
                                return Err(e);
                            }
                        }
                        Ok(local_ctx)
                    });
                    handles.push(handle);
                }

                // Wait for all items to complete
                let mut results = Vec::new();
                for handle in handles {
                    match handle.await {
                        Ok(Ok(item_ctx)) => results.push(item_ctx),
                        Ok(Err(e)) => return Err(e),
                        Err(e) => return Err(anyhow::anyhow!("Parallel_for execution failed: {}", e)),
                    }
                }

                // Merge results back into main context
                for (i, item_ctx) in results.into_iter().enumerate() {
                    ctx["ctx"][format!("{}_{}", merge_key, i)] = item_ctx["ctx"].clone();
                }
            }

            // Handle parallel execution step (static branches)
            if let Some(parallel) = step.get("parallel") {
                let branches: Vec<Json> = parallel.get("branches").and_then(|v| v.as_array()).context("branches")?.iter().map(|b| b.clone()).collect();

                // Execute all branches in parallel
                let mut handles = Vec::new();
                for (i, branch) in branches.iter().enumerate() {
                    let branch_ctx = ctx.clone();
                    let instance_id = id;
                    let db = self.db.clone();
                    let branch_clone = branch.clone();

                    let handle = tokio::spawn(async move {
                        let mut local_ctx = branch_ctx;
                        if let Some(branch_steps) = branch_clone.get("steps").and_then(|v| v.as_array()) {
                            let branch_steps_clone: Vec<Json> = branch_steps.iter().map(|s| s.clone()).collect();
                            for branch_step in branch_steps_clone {
                                let interpreter = Interpreter { db: &db };
                                if let Err(e) = interpreter.execute_step(branch_step.clone(), &mut local_ctx, instance_id).await {
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
                                        self.execute_step(case_step.clone(), &mut ctx, id).await?;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Handle DAG step (basic)
            if let Some(dag) = step.get("dag") {
                let nodes = dag.get("nodes").and_then(|v| v.as_array()).context("dag nodes")?;
                let edges = dag.get("edges").and_then(|v| v.as_array()).context("dag edges")?;

                use std::collections::{HashMap, HashSet, VecDeque};
                let mut indegree: HashMap<String, usize> = HashMap::new();
                let mut node_map: HashMap<String, &Json> = HashMap::new();
                for n in nodes {
                    if let Some(id) = n.get("id").and_then(|v| v.as_str()) { indegree.insert(id.to_string(), 0); node_map.insert(id.to_string(), n); }
                }
                for e in edges {
                    if let (Some(from), Some(to)) = (e.get("from").and_then(|v| v.as_str()), e.get("to").and_then(|v| v.as_str())) {
                        *indegree.entry(to.to_string()).or_insert(0) += 1;
                    }
                }
                let mut q: VecDeque<String> = indegree.iter().filter_map(|(k,&v)| if v==0 { Some(k.clone()) } else { None }).collect();
                let mut processed: HashSet<String> = HashSet::new();
                while let Some(idn) = q.pop_front() {
                    processed.insert(idn.clone());
                    if let Some(node) = node_map.get(&idn) {
                        if let Some(steps) = node.get("steps").and_then(|v| v.as_array()) {
                            for s in steps { self.execute_step(s.clone(), &mut ctx, id).await?; }
                        }
                    }
                    for e in edges {
                        if e.get("from").and_then(|v| v.as_str()) == Some(idn.as_str()) {
                            if let Some(to) = e.get("to").and_then(|v| v.as_str()) {
                                if let Some(entry) = indegree.get_mut(to) { if *entry>0 { *entry -= 1; if *entry==0 { q.push_back(to.to_string()); }}}
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
        })
    }

    fn execute_step<'b>(&'b self, step: Json, ctx: &'b mut Json, instance_id: Uuid) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'b>> {
        Box::pin(async move {
        let store = InstanceStore { db: self.db };

        // try/catch/finally inside nested execution
        if let Some(try_block) = step.get("try") {
            let mut error_raised: Option<anyhow::Error> = None;
            if let Some(try_steps) = try_block.get("steps").and_then(|v| v.as_array()) {
                for try_step in try_steps {
                    if let Err(e) = self.execute_step(try_step.clone(), ctx, instance_id).await { error_raised = Some(e); break; }
                }
            }
            if let Some(err) = error_raised {
                if let Some(catches) = step.get("catch").and_then(|v| v.as_array()) {
                    let mut handled = false;
                    for c in catches {
                        let on = c.get("on").and_then(|v| v.as_str()).unwrap_or("any");
                        let matches = matches!(on, "any" | "http_error" | "kafka_error" | "timeout");
                        if matches {
                            if let Some(steps) = c.get("steps").and_then(|v| v.as_array()) {
                                for s in steps { let _ = self.execute_step(s.clone(), ctx, instance_id).await; }
                            }
                            handled = true; break;
                        }
                    }
                    if !handled { return Err(err); }
                } else { return Err(err); }
            }
            if let Some(finally) = step.get("finally").and_then(|v| v.as_array()) {
                for s in finally { let _ = self.execute_step(s.clone(), ctx, instance_id).await; }
            }
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
                instance_id,
                step_id,
                "forward",
                body,
                None,
            ).await?;

            // Execute HTTP call with timeout + retry policies
            match self.exec_with_policies_http(&step, method, url, body, instance_id, step_id).await {
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

            // Execute Kafka publish with timeout + retry policies
            match self.exec_with_policies_kafka(&step, topic, key, &payload, instance_id, step_id).await {
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
        })
    }

    fn read_timeout_secs(step: &Json, nested_key: &str) -> Option<u64> {
        step.get(nested_key)
            .and_then(|v| v.get("timeout_secs").and_then(|v| v.as_u64()))
            .or_else(|| step.get("timeout_secs").and_then(|v| v.as_u64()))
    }

    fn read_retry(step: &Json) -> (u32, u64, u64) {
        if let Some(r) = step.get("retry") {
            let max_attempts = r.get("max_attempts").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
            let initial = r.get("backoff").and_then(|b| b.get("initial_secs")).and_then(|v| v.as_u64()).unwrap_or(1);
            let max = r.get("backoff").and_then(|b| b.get("max_secs")).and_then(|v| v.as_u64()).unwrap_or(30);
            (max_attempts, initial, max)
        } else {
            (1, 1, 30)
        }
    }

    async fn exec_with_policies_http(
        &self,
        step: &Json,
        method: &str,
        url: &str,
        body: Option<&Json>,
        instance_id: Uuid,
        step_id: &str,
    ) -> anyhow::Result<Json> {
        // Circuit breaker
        let service_name = step.get("resilience").and_then(|v| v.get("circuit")).and_then(|v| v.get("service")).and_then(|v| v.as_str()).unwrap_or("");
        if !service_name.is_empty() && !resilience::can_call(service_name) {
            return Err(anyhow::anyhow!("circuit_open"));
        }
        let timeout_secs = Self::read_timeout_secs(step, "http");
        let (max_attempts, initial, max) = Self::read_retry(step);
        let mut attempt: u32 = 0;
        let mut backoff = initial;
        loop {
            attempt += 1;
            let fut = http_client::call(method, url, body);
            let result = if let Some(ts) = timeout_secs { timeout(Duration::seconds(ts as i64).to_std().unwrap(), fut).await.map_err(|_| anyhow::anyhow!("timeout"))? } else { fut.await };
            match result {
                Ok(resp) => {
                    if !service_name.is_empty() { resilience::record_success(service_name); }
                    return Ok(resp)
                },
                Err(e) => {
                    if attempt >= max_attempts { return Err(e); }
                    // log and backoff
                    SagaLogStore { db: self.db }.log(
                        instance_id,
                        step_id,
                        "retry",
                        None,
                        Some(&serde_json::json!({"attempt": attempt, "error": e.to_string()})),
                    ).await.ok();
                    if !service_name.is_empty() { resilience::record_failure(service_name); }
                    sleep(std::time::Duration::from_secs(backoff)).await;
                    backoff = (backoff * 2).min(max);
                }
            }
        }
    }

    async fn exec_with_policies_kafka(
        &self,
        step: &Json,
        topic: &str,
        key: &str,
        payload: &Json,
        instance_id: Uuid,
        step_id: &str,
    ) -> anyhow::Result<()> {
        let timeout_secs = Self::read_timeout_secs(step, "kafka-publish");
        let (max_attempts, initial, max) = Self::read_retry(step);
        let mut attempt: u32 = 0;
        let mut backoff = initial;
        loop {
            attempt += 1;
            let fut = kafka::publish(topic, key, payload);
            let result = if let Some(ts) = timeout_secs { timeout(Duration::seconds(ts as i64).to_std().unwrap(), fut).await.map_err(|_| anyhow::anyhow!("timeout"))? } else { fut.await };
            match result {
                Ok(()) => return Ok(()),
                Err(e) => {
                    if attempt >= max_attempts { return Err(e); }
                    SagaLogStore { db: self.db }.log(
                        instance_id,
                        step_id,
                        "retry",
                        None,
                        Some(&serde_json::json!({"attempt": attempt, "error": e.to_string()})),
                    ).await.ok();
                    sleep(std::time::Duration::from_secs(backoff)).await;
                    backoff = (backoff * 2).min(max);
                }
            }
        }
    }
}
