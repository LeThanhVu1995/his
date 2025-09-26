use actix_web::{web, HttpResponse};
use serde_json::Value as Json;
use uuid::Uuid;
use crate::store::instances::InstanceStore;
use crate::engine::interpreter::Interpreter;

pub async fn handle_event(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<Json>,
) -> actix_web::Result<HttpResponse> {
    let event_name = body.get("event").and_then(|v| v.as_str()).unwrap_or("");
    let payload = body.get("payload").cloned().unwrap_or(Json::Null);
    let correlation_id = body.get("correlation_id").and_then(|v| v.as_str());

    tracing::info!("Received event: {} with correlation_id: {:?}", event_name, correlation_id);

    // Find waiting instances for this event
    let store = InstanceStore { db: &db };
    let waiting_instances = find_waiting_instances(&store, event_name).await
        .map_err(|_| crate::error::AppError::Internal("Failed to find waiting instances".into()))?;

    // Process each waiting instance
    for instance_id in waiting_instances {
        if let Some(instance) = store.get(instance_id).await
            .map_err(|_| crate::error::AppError::Internal("Failed to get instance".into()))? {

            // Update context with event payload
            let mut ctx = instance.context.clone();
            if let Some(response_key) = instance.cursor.get("response_key").and_then(|v| v.as_str()) {
                ctx["ctx"][response_key] = payload.clone();
            }

            // Clear waiting state
            let cursor = serde_json::json!({"step": instance.cursor.get("step").unwrap_or(&Json::Number(serde_json::Number::from(0)))});
            store.save_progress(instance_id, &cursor, &ctx, "RUNNING", None, None).await
                .map_err(|_| crate::error::AppError::Internal("Failed to update instance".into()))?;

            // Resume execution
            let interpreter = Interpreter { db: &db };
            if let Err(e) = interpreter.tick(instance_id).await {
                tracing::error!("Failed to resume instance {}: {}", instance_id, e);
            }
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "processed"})))
}

async fn find_waiting_instances(
    store: &InstanceStore<'_>,
    event_name: &str,
) -> anyhow::Result<Vec<Uuid>> {
    // This would typically query the database for instances waiting for this specific event
    // For now, we'll get all waiting instances and filter by event name
    let waiting_instances = store.list_waiting().await?;

    let mut matching_instances = Vec::new();
    for instance_id in waiting_instances {
        if let Some(instance) = store.get(instance_id).await? {
            if let Some(waiting_for_event) = instance.cursor.get("waiting_for_event").and_then(|v| v.as_str()) {
                if waiting_for_event == event_name {
                    matching_instances.push(instance_id);
                }
            }
        }
    }

    Ok(matching_instances)
}
