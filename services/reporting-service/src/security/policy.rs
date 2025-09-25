pub mod perm {
    pub const DASHBOARD_VIEW: &str = "his.reporting.dashboard.view";
    pub const QUERY_RUN: &str = "his.reporting.query.run";
    pub const EXPORT: &str = "his.reporting.export";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(DASHBOARD_VIEW, "View reporting dashboards", "dashboards", "view"),
        PermissionDef::new(QUERY_RUN, "Run ad-hoc queries", "queries", "run"),
        PermissionDef::new(EXPORT, "Export reports", "exports", "export"),
    ]
}


