// src/extractors/pagination.rs placeholder
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use serde::Deserialize;

/// Query mẫu: ?page=1&page_size=50&sort=created_at&order=desc
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort: Option<String>,
    pub order: Option<String>, // "asc" | "desc"
}

#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub offset: u64,
    pub limit: u64,
}

impl Pagination {
    pub const DEFAULT_PAGE: u32 = 1;
    pub const DEFAULT_SIZE: u32 = 20;
    pub const MAX_SIZE: u32 = 200;

    pub fn from_query(q: &PaginationQuery) -> Self {
        let mut page = q.page.unwrap_or(Self::DEFAULT_PAGE);
        let mut size = q.page_size.unwrap_or(Self::DEFAULT_SIZE);

        if page == 0 { page = 1; }
        if size == 0 { size = Self::DEFAULT_SIZE; }
        if size > Self::MAX_SIZE { size = Self::MAX_SIZE; }

        let offset = ((page - 1) as u64) * (size as u64);
        let limit = size as u64;

        Self { page, page_size: size, offset, limit }
    }
}

impl FromRequest for Pagination {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let q = actix_web::web::Query::<PaginationQuery>::from_query(req.query_string())
            .map(|q| q.into_inner())
            .unwrap_or_default();

        ready(Ok(Pagination::from_query(&q)))
    }
}
