use crate::infra::opensearch::client::OsClient;
use serde_json::Value as Json;

pub async fn run(db: sqlx::Pool<sqlx::Postgres>) {
    let os = OsClient::from_env();
    let _ = db;
    let _ = os;
    // TODO: subscribe to kafka and handle events
}

pub async fn index_patient(os: &OsClient, payload: &Json) -> anyhow::Result<()> {
    let idx = "patients";
    let id = payload["id"].as_str().unwrap_or("");
    os.upsert_doc(idx, id, payload).await
}
