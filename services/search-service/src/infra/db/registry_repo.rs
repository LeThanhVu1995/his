use uuid::Uuid;
use sqlx::{Pool, Postgres};

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct IndexReg {
    pub id: Uuid,
    pub index_code: String,
    pub index_name: String,
    pub mapping: serde_json::Value,
    pub settings: Option<serde_json::Value>,
}

pub struct RegistryRepo<'a> { pub db: &'a Pool<Postgres> }

impl<'a> RegistryRepo<'a> {
    pub async fn upsert(&self, code: &str, name: &str, mapping: &serde_json::Value, settings: Option<&serde_json::Value>) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO index_registry(id,index_code,index_name,mapping,settings)
               VALUES($1,$2,$3,$4,$5)
               ON CONFLICT(index_code) DO UPDATE SET
                 index_name=EXCLUDED.index_name,
                 mapping=EXCLUDED.mapping,
                 settings=EXCLUDED.settings,
                 updated_at=NOW()"#
        )
        .bind(uuid::Uuid::new_v4())
        .bind(code)
        .bind(name)
        .bind(mapping)
        .bind(settings)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, code: &str) -> anyhow::Result<Option<IndexReg>> {
        Ok(sqlx::query_as::<_, IndexReg>(
            r#"SELECT id,index_code,index_name,mapping,settings FROM index_registry WHERE index_code=$1"#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?)
    }
}
