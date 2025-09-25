pub mod perm {
    pub const SEARCH_QUERY: &str = "his.search.query";
    pub const SEARCH_REINDEX: &str = "his.search.reindex";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(SEARCH_QUERY, "Query across indices", "search", "query"),
        PermissionDef::new(SEARCH_REINDEX, "Reindex search data", "search", "reindex"),
    ]
}


