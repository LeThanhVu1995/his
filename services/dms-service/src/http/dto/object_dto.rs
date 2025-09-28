use serde::{Deserialize,Serialize}; use uuid::Uuid;

#[derive(Debug,Deserialize)]
pub struct CreatePresignReq{ pub name:String, pub content_type:String, pub category:Option<String>, pub entity_type:Option<String>, pub entity_id:Option<Uuid> }

#[derive(Debug,Serialize)]
pub struct CreatePresignRes{ pub id:Uuid, pub key:String, pub upload_url:String }

#[derive(Debug,Deserialize)]
pub struct GetUrlReq{ pub key:String }

#[derive(Debug,Serialize)]
pub struct GetUrlRes{ pub download_url:String }
// dms-service src/http/dto/object_dto.rs placeholder
