use crate::infra::storage::minio::S3;
use uuid::Uuid;

pub struct PresignSvc{
  pub s3:S3,
  pub db: sqlx::Pool<sqlx::Postgres>
}

impl PresignSvc{
  pub async fn create_record_and_presign_put(
    &self,
    name:&str,
    content_type:&str,
    category:Option<&str>,
    entity:(Option<&str>, Option<Uuid>),
    user:Option<Uuid>
  ) -> anyhow::Result<(Uuid, String, String)>{
    let id=Uuid::new_v4();
    let key=format!("{}/{}", chrono::Utc::now().format("%Y/%m/%d"), id);

    // create DB record (size/sha256 sẽ cập nhật sau khi client callback)
    sqlx::query(
      "INSERT INTO dms_objects(id,bucket,object_key,content_type,name,category,entity_type,entity_id,created_by) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"
    )
    .bind(id)
    .bind(std::env::var("S3_BUCKET")?)
    .bind(&key)
    .bind(content_type)
    .bind(name)
    .bind(category.map(|s|s.to_string()))
    .bind(entity.0.map(|s|s.to_string()))
    .bind(entity.1)
    .bind(user)
    .execute(&self.db).await?;

    let url=self.s3.presign_put(
      &key,
      std::env::var("PRESIGN_EXPIRES_SECS").ok().and_then(|v|v.parse().ok()).unwrap_or(900),
      content_type
    ).await?;
    Ok((id, key, url))
  }

  pub async fn presign_get(&self, key:&str)->anyhow::Result<String>{
    Ok(self.s3.presign_get(
      key,
      std::env::var("PRESIGN_EXPIRES_SECS").ok().and_then(|v|v.parse().ok()).unwrap_or(900)
    ).await?)
  }
}
