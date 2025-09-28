use serde::Deserialize; use uuid::Uuid;

#[derive(Debug,Deserialize)]
pub struct AttachSignReq{ pub object_id:Uuid, pub signer_id:Option<Uuid>, pub signer_name:Option<String>, pub signature_alg:String, pub signature_b64:String, pub note:Option<String> }
// dms-service src/http/dto/sign_dto.rs placeholder
