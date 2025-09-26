#[derive(Clone)]
pub struct OsClient {
    base: String,
    user: Option<String>,
    pass: Option<String>,
    http: reqwest::Client,
}

impl OsClient {
    pub fn from_env() -> Self {
        Self {
            base: std::env::var("OPENSEARCH_URL").unwrap_or("http://localhost:9200".into()),
            user: std::env::var("OPENSEARCH_USER").ok(),
            pass: std::env::var("OPENSEARCH_PASS").ok(),
            http: reqwest::Client::new(),
        }
    }

    async fn req(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}/{}", self.base.trim_end_matches('/'), path.trim_start_matches('/'));
        let mut r = self.http.request(method, url);
        if let (Some(u), Some(p)) = (self.user.as_ref(), self.pass.as_ref()) {
            r = r.basic_auth(u, Some(p));
        }
        r
    }

    pub async fn create_index(&self, name: &str, body: &serde_json::Value) -> anyhow::Result<()> {
        let res = self
            .req(reqwest::Method::PUT, &format!("{}", name))
            .await
            .json(body)
            .send()
            .await?;
        if !res.status().is_success() {
            anyhow::bail!(format!("os create index {:?}", res.text().await?));
        }
        Ok(())
    }

    pub async fn upsert_doc(&self, index: &str, id: &str, doc: &serde_json::Value) -> anyhow::Result<()> {
        let res = self
            .req(reqwest::Method::PUT, &format!("{}/_doc/{}", index, id))
            .await
            .json(doc)
            .send()
            .await?;
        if !res.status().is_success() {
            anyhow::bail!(res.text().await?);
        }
        Ok(())
    }

    pub async fn search(&self, index: &str, q: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
        let res = self
            .req(reqwest::Method::GET, &format!("{}/_search", index))
            .await
            .json(q)
            .send()
            .await?;
        Ok(res.json::<serde_json::Value>().await?)
    }

    pub async fn delete_doc(&self, index: &str, id: &str) -> anyhow::Result<()> {
        let res = self
            .req(reqwest::Method::DELETE, &format!("{}/_doc/{}", index, id))
            .await
            .send()
            .await?;
        if !res.status().is_success() {
            anyhow::bail!(res.text().await?);
        }
        Ok(())
    }
}
