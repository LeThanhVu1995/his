use uuid::Uuid; use sqlx::{Pool,Postgres}; use crate::domain::entities::object::DmsObject;
pub struct ObjectRepo<'a>{ pub db:&'a Pool<Postgres> }
impl<'a> ObjectRepo<'a>{
  pub async fn insert(&self, o:&DmsObject)->anyhow::Result<()> { sqlx::query("INSERT INTO dms_objects(id,bucket,object_key,content_type,size,sha256,name,category,entity_type,entity_id,created_by) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)").bind(o.id).bind(&o.bucket).bind(&o.object_key).bind(&o.content_type).bind(&o.size).bind(&o.sha256).bind(&o.name).bind(&o.category).bind(&o.entity_type).bind(&o.entity_id).bind(&o.created_by).execute(self.db).await?; Ok(()) }
  pub async fn get(&self, id:Uuid)->anyhow::Result<Option<DmsObject>>{ Ok(sqlx::query_as::<_,DmsObject>(r#"SELECT id,bucket,object_key,content_type,size,sha256,name,category,entity_type,entity_id,created_by,created_at,updated_at FROM dms_objects WHERE id=$1"#).bind(id).fetch_optional(self.db).await?) }
  pub async fn by_entity(&self, et:&str, eid:Uuid)->anyhow::Result<Vec<DmsObject>>{ Ok(sqlx::query_as::<_,DmsObject>(r#"SELECT id,bucket,object_key,content_type,size,sha256,name,category,entity_type,entity_id,created_by,created_at,updated_at FROM dms_objects WHERE entity_type=$1 AND entity_id=$2 ORDER BY created_at DESC"#).bind(et).bind(eid).fetch_all(self.db).await?) }
}
