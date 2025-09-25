pub struct SearchSvc { pub os: crate::infra::opensearch::client::OsClient }

impl SearchSvc {
    pub fn new(os: crate::infra::opensearch::client::OsClient) -> Self { Self { os } }

    pub async fn search_all(&self, q: &str, limit: i64) -> anyhow::Result<serde_json::Value> {
        let body = serde_json::json!({
          "size": limit.clamp(1, 200),
          "query": { "multi_match": { "query": q, "fields": ["full_name^3","name^2","code","text","tags"] } }
        });
        let res = self.os.search("patients,encounters,orders,items,docs", &body).await?;
        Ok(res)
    }
}
