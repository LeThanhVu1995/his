pub async fn call(
    method: &str,
    url: &str,
    body: Option<&serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    let c = reqwest::Client::new();
    let mut req = c.request(method.parse()?, url);
    if let Some(b) = body {
        req = req.json(b);
    }
    let res = req.send().await?;
    let j = res.json::<serde_json::Value>().await.unwrap_or(serde_json::json!({"ok": true}));
    Ok(j)
}
