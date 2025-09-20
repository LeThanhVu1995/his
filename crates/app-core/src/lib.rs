pub mod pagination;
pub mod time;

#[cfg(feature = "sqlx")]
pub mod result_ext;

pub mod prelude {
    pub use crate::pagination::{PageInfo, PageParams, Paged, PagedResponse, PaginationClamp, PaginationMeta};
    pub use crate::time::{now_utc, parse_rfc3339_to_utc, start_of_day_utc, end_of_day_utc};

    #[cfg(feature = "sqlx")]
    pub use crate::result_ext::{OptionExt, ResultExt};
}
