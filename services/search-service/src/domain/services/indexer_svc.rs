use crate::infra::{opensearch::client::OsClient, db::registry_repo::RegistryRepo};
use sqlx::Row;

pub struct IndexerSvc<'a> { pub db: &'a sqlx::Pool<sqlx::Postgres>, pub os: OsClient }

impl<'a> IndexerSvc<'a> {
    pub async fn ensure_index(&self, code: &str) -> anyhow::Result<String> {
        let reg = RegistryRepo { db: self.db }.get(code).await?.ok_or(anyhow::anyhow!("no registry"))?;
        let body = serde_json::json!({ "settings": reg.settings.clone().unwrap_or(serde_json::json!({})), "mappings": reg.mapping.clone() });
        let _ = self.os.create_index(&reg.index_name, &body).await; // idempotent best-effort
        Ok(reg.index_name)
    }

    pub async fn reindex_patients(&self) -> anyhow::Result<u64> {
        let idx = self.ensure_index("patients").await?;
        let rows = sqlx::query(
            r#"SELECT id::text, code, full_name, dob, gender, phone, id_no, address FROM staging_patients"#
        ).fetch_all(self.db).await?;
        let mut ok = 0u64;
        for r in rows {
            let id: Option<String> = r.try_get("id").ok();
            let code: Option<String> = r.try_get("code").ok();
            let full_name: Option<String> = r.try_get("full_name").ok();
            let dob: Option<chrono::NaiveDate> = r.try_get("dob").ok();
            let gender: Option<String> = r.try_get("gender").ok();
            let phone: Option<String> = r.try_get("phone").ok();
            let id_no: Option<String> = r.try_get("id_no").ok();
            let address: Option<String> = r.try_get("address").ok();
            let doc = serde_json::json!({
                "id": id,
                "code": code,
                "full_name": full_name,
                "dob": dob,
                "gender": gender,
                "phone": phone,
                "id_no": id_no,
                "address": address
            });
            if self.os.upsert_doc(&idx, &id.clone().unwrap_or_default(), &doc).await.is_ok() { ok += 1; }
        }
        Ok(ok)
    }
}
