pub async fn call(
    service: &str,
    method: &str,
    payload: &serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    // TODO: Implement gRPC client calls
    tracing::debug!("gRPC call: {}.{}", service, method);
    Ok(serde_json::json!({"ok": true}))
}
