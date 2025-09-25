pub fn patient_mapping() -> serde_json::Value {
    serde_json::json!({
      "settings": { "analysis": { "analyzer": { "vn_text": { "type":"standard" } } } },
      "mappings": {
        "properties": {
          "id": {"type":"keyword"},
          "code": {"type":"keyword"},
          "full_name": {"type":"text","analyzer":"vn_text","fields":{"keyword":{"type":"keyword"}}},
          "dob": {"type":"date"},
          "gender": {"type":"keyword"},
          "phone": {"type":"keyword"},
          "id_no": {"type":"keyword"},
          "address": {"type":"text","analyzer":"vn_text"}
        }
      }
    })
}

pub fn generic_mapping() -> serde_json::Value {
    serde_json::json!({
      "settings": { "analysis": { "analyzer": { "vn_text": { "type":"standard" } } } },
      "mappings": { "properties": { "id": {"type":"keyword"}, "code": {"type":"keyword"}, "name": {"type":"text","analyzer":"vn_text","fields":{"keyword":{"type":"keyword"}}}, "text": {"type":"text","analyzer":"vn_text"}, "tags": {"type":"keyword"}, "created_at": {"type":"date"} } }
    })
}
// search-service src/infra/opensearch/mappings.rs placeholder
