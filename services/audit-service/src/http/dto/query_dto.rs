use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Page {
    pub page: Option<i64>,
    pub page_size: Option<i64>
}

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub user_id: String,
    pub page: Option<i64>,
    pub page_size: Option<i64>
}

#[derive(Debug, Deserialize)]
pub struct EntityQuery {
    pub entity_name: String,
    pub entity_id: String,
    pub page: Option<i64>,
    pub page_size: Option<i64>
}

#[derive(Debug, Deserialize)]
pub struct ActionQuery {
    pub action: String,
    pub page: Option<i64>,
    pub page_size: Option<i64>
}


