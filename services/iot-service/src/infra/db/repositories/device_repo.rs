use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::device::Device;

pub struct DeviceRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DeviceRepo<'a> {
    pub async fn upsert(&self, code: &str, name: &str, r#type: &str, location: Option<&str>) -> anyhow::Result<Device> {
        let rec = sqlx::query_as::<_, Device>(
            r#"INSERT INTO iot_devices(id,code,name,type,location) VALUES($1,$2,$3,$4,$5)
               ON CONFLICT(code) DO UPDATE SET name=EXCLUDED.name, type=EXCLUDED.type, location=EXCLUDED.location
               RETURNING id,code,name,type as "r#type!",location,last_seen,created_at"#
        )
        .bind(uuid::Uuid::new_v4())
        .bind(code)
        .bind(name)
        .bind(r#type)
        .bind(location)
        .fetch_one(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn get_by_code(&self, code: &str) -> anyhow::Result<Option<Device>> {
        Ok(sqlx::query_as::<_, Device>(
            r#"SELECT id,code,name,type as "r#type!",location,last_seen,created_at FROM iot_devices WHERE code=$1"#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Device>> {
        Ok(sqlx::query_as::<_, Device>(
            r#"SELECT id,code,name,type as "r#type!",location,last_seen,created_at FROM iot_devices WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<Device>> {
        let devices = sqlx::query_as::<_, Device>(
            r#"SELECT id,code,name,type as "r#type!",location,last_seen,created_at FROM iot_devices
               ORDER BY created_at DESC LIMIT $1 OFFSET $2"#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(devices)
    }

    pub async fn touch_seen(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("UPDATE iot_devices SET last_seen=NOW() WHERE id=$1")
            .bind(id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
