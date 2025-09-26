pub struct SearchSvc { pub os: crate::infra::opensearch::client::OsClient }

impl SearchSvc {
    pub fn new(os: crate::infra::opensearch::client::OsClient) -> Self { Self { os } }

    pub async fn search_all(&self, q: &str, limit: i64) -> anyhow::Result<serde_json::Value> {
        let body = serde_json::json!({
          "size": limit.clamp(1, 200),
          "query": {
            "multi_match": {
              "query": q,
              "fields": [
                "full_name^3",
                "name^2",
                "code",
                "text",
                "tags",
                "file_name^2",
                "department_name",
                "room_name",
                "attending_staff",
                "remarks",
                "note"
              ]
            }
          }
        });
        let res = self.os.search("his-patients-v1,his-encounters-v1,his-orders-v1,his-documents-v1", &body).await?;
        Ok(res)
    }
}
