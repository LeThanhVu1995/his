use crate::infra::opensearch::client::OsClient;
use crate::domain::services::indexer_svc::IndexerSvc;
use serde_json::Value as Json;

pub async fn run(db: sqlx::Pool<sqlx::Postgres>) {
    let os = OsClient::from_env();
    let _indexer = IndexerSvc { db: &db, os: os.clone() };

    tracing::info!("Starting Kafka consumer for search-service auto-sync");

    // Mock Kafka consumer - in real implementation, this would connect to actual Kafka
    // For now, we'll simulate periodic checks for demonstration
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        // In real implementation, this would be triggered by Kafka events
        // For now, we'll just log that we're running
        tracing::debug!("Kafka consumer is running (mock implementation)");
    }
}

pub async fn handle_database_event(
    event_type: &str,
    entity_type: &str,
    entity_id: &str,
    payload: &Json,
    db: &sqlx::Pool<sqlx::Postgres>,
    os: &OsClient,
) -> anyhow::Result<()> {
    let indexer = IndexerSvc { db, os: os.clone() };

    match (event_type, entity_type) {
        ("CREATE", "patient") | ("UPDATE", "patient") => {
            handle_patient_event(entity_id, payload, &indexer).await?;
        },
        ("DELETE", "patient") => {
            handle_patient_delete(entity_id, os).await?;
        },
        ("CREATE", "encounter") | ("UPDATE", "encounter") => {
            handle_encounter_event(entity_id, payload, &indexer).await?;
        },
        ("DELETE", "encounter") => {
            handle_encounter_delete(entity_id, os).await?;
        },
        ("CREATE", "clinical_order") | ("UPDATE", "clinical_order") => {
            handle_order_event(entity_id, payload, &indexer).await?;
        },
        ("DELETE", "clinical_order") => {
            handle_order_delete(entity_id, os).await?;
        },
        ("CREATE", "doc_file") | ("UPDATE", "doc_file") => {
            handle_document_event(entity_id, payload, &indexer).await?;
        },
        ("DELETE", "doc_file") => {
            handle_document_delete(entity_id, os).await?;
        },
        _ => {
            tracing::debug!("Unhandled event: {} for entity: {}", event_type, entity_type);
        }
    }

    Ok(())
}

async fn handle_patient_event(entity_id: &str, payload: &Json, indexer: &IndexerSvc<'_>) -> anyhow::Result<()> {
    let idx = indexer.ensure_index("patients").await?;

    // Transform payload to match search index format
    let doc = serde_json::json!({
        "id": entity_id,
        "code": payload.get("code"),
        "full_name": payload.get("full_name"),
        "date_of_birth": payload.get("date_of_birth"),
        "gender": payload.get("gender"),
        "phone_number": payload.get("phone_number"),
        "national_id": payload.get("national_id"),
        "address": format!("{} {} {} {}",
            payload.get("address_line1").unwrap_or(&Json::Null),
            payload.get("address_line2").unwrap_or(&Json::Null),
            payload.get("city").unwrap_or(&Json::Null),
            payload.get("province").unwrap_or(&Json::Null)
        ),
        "email": payload.get("email"),
        "status": payload.get("status")
    });

    indexer.os.upsert_doc(&idx, entity_id, &doc).await?;
    tracing::info!("Indexed patient: {}", entity_id);
    Ok(())
}

async fn handle_patient_delete(entity_id: &str, os: &OsClient) -> anyhow::Result<()> {
    os.delete_doc("his-patients-v1", entity_id).await?;
    tracing::info!("Deleted patient from index: {}", entity_id);
    Ok(())
}

async fn handle_encounter_event(entity_id: &str, payload: &Json, indexer: &IndexerSvc<'_>) -> anyhow::Result<()> {
    let idx = indexer.ensure_index("encounters").await?;

    let doc = serde_json::json!({
        "id": entity_id,
        "patient_id": payload.get("patient_id"),
        "encounter_id": entity_id,
        "type_code": payload.get("type_code"),
        "status": payload.get("status"),
        "start_time": payload.get("start_time"),
        "end_time": payload.get("end_time"),
        "department_name": payload.get("department_name"),
        "room_name": payload.get("room_name"),
        "attending_staff": payload.get("attending_staff")
    });

    indexer.os.upsert_doc(&idx, entity_id, &doc).await?;
    tracing::info!("Indexed encounter: {}", entity_id);
    Ok(())
}

async fn handle_encounter_delete(entity_id: &str, os: &OsClient) -> anyhow::Result<()> {
    os.delete_doc("his-encounters-v1", entity_id).await?;
    tracing::info!("Deleted encounter from index: {}", entity_id);
    Ok(())
}

async fn handle_order_event(entity_id: &str, payload: &Json, indexer: &IndexerSvc<'_>) -> anyhow::Result<()> {
    let idx = indexer.ensure_index("orders").await?;

    let doc = serde_json::json!({
        "id": entity_id,
        "order_id": entity_id,
        "patient_id": payload.get("patient_id"),
        "encounter_id": payload.get("encounter_id"),
        "order_type": payload.get("order_type"),
        "status": payload.get("status"),
        "priority_code": payload.get("priority_code"),
        "ordered_at": payload.get("ordered_at"),
        "remarks": payload.get("remarks")
    });

    indexer.os.upsert_doc(&idx, entity_id, &doc).await?;
    tracing::info!("Indexed order: {}", entity_id);
    Ok(())
}

async fn handle_order_delete(entity_id: &str, os: &OsClient) -> anyhow::Result<()> {
    os.delete_doc("his-orders-v1", entity_id).await?;
    tracing::info!("Deleted order from index: {}", entity_id);
    Ok(())
}

async fn handle_document_event(entity_id: &str, payload: &Json, indexer: &IndexerSvc<'_>) -> anyhow::Result<()> {
    let idx = indexer.ensure_index("documents").await?;

    let doc = serde_json::json!({
        "id": entity_id,
        "doc_id": entity_id,
        "file_name": payload.get("file_name"),
        "mime_type": payload.get("mime_type"),
        "entity_name": payload.get("entity_name"),
        "entity_id": payload.get("entity_id"),
        "uploaded_at": payload.get("uploaded_at"),
        "uploaded_by": payload.get("uploaded_by"),
        "note": payload.get("note")
    });

    indexer.os.upsert_doc(&idx, entity_id, &doc).await?;
    tracing::info!("Indexed document: {}", entity_id);
    Ok(())
}

async fn handle_document_delete(entity_id: &str, os: &OsClient) -> anyhow::Result<()> {
    os.delete_doc("his-documents-v1", entity_id).await?;
    tracing::info!("Deleted document from index: {}", entity_id);
    Ok(())
}

// Legacy function for backward compatibility
pub async fn index_patient(os: &OsClient, payload: &Json) -> anyhow::Result<()> {
    let idx = "his-patients-v1";
    let id = payload["id"].as_str().unwrap_or("");
    os.upsert_doc(idx, id, payload).await
}
