// src/pagination.rs placeholder
use serde::{Deserialize, Serialize};

/// Tham số phân trang mức đầu vào (page/size). Không phụ thuộc web framework.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PageParams {
    pub page: u32,
    pub page_size: u32,
}

impl Default for PageParams {
    fn default() -> Self {
        Self { page: 1, page_size: 20 }
    }
}

/// Ràng buộc/giới hạn phân trang (ví dụ size tối đa).
#[derive(Debug, Clone, Copy)]
pub struct PaginationClamp {
    pub min_page: u32,
    pub default_page: u32,
    pub min_size: u32,
    pub default_size: u32,
    pub max_size: u32,
}

impl Default for PaginationClamp {
    fn default() -> Self {
        Self {
            min_page: 1,
            default_page: 1,
            min_size: 1,
            default_size: 20,
            max_size: 200,
        }
    }
}

impl PageParams {
    /// Áp ràng buộc hợp lệ và tính offset/limit.
    pub fn clamp(self, clamp: PaginationClamp) -> (u32, u32, u64, u64) {
        let mut page = if self.page == 0 { clamp.default_page } else { self.page };
        if page < clamp.min_page { page = clamp.min_page; }

        let mut size = if self.page_size == 0 { clamp.default_size } else { self.page_size };
        if size < clamp.min_size { size = clamp.min_size; }
        if size > clamp.max_size { size = clamp.max_size; }

        let offset = ((page - 1) as u64) * size as u64;
        let limit = size as u64;
        (page, size, offset, limit)
    }

    pub fn limit_offset(self, clamp: PaginationClamp) -> (u64, u64) {
        let (_p, _s, offset, limit) = self.clamp(clamp);
        (limit, offset)
    }
}

/// Thông tin meta trả về cho client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageInfo {
    pub page: u32,
    pub page_size: u32,
    pub total_items: u64,
    pub total_pages: u32,
}

/// Dạng response chuẩn: `data` + `meta`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Paged<T> {
    pub data: Vec<T>,
    pub meta: PageInfo,
}

/// Alias hay dùng cho HTTP JSON trả ra.
pub type PagedResponse<T> = Paged<T>;

/// Backward-compat alias for meta info naming used elsewhere.
pub type PaginationMeta = PageInfo;

/// Meta skeleton nếu bạn chỉ biết `total_items` và `PageParams`.
pub fn make_page_info(total_items: u64, page: u32, page_size: u32) -> PageInfo {
    let total_pages = if page_size == 0 {
        0
    } else {
        // ceil(total_items / page_size)
        ((total_items + (page_size as u64) - 1) / (page_size as u64)) as u32
    };
    PageInfo { page, page_size, total_items, total_pages }
}

/// Helper để bọc dữ liệu và meta thành `Paged<T>`.
pub fn to_paged<T>(items: Vec<T>, total_items: u64, page: u32, page_size: u32) -> Paged<T> {
    Paged {
        data: items,
        meta: make_page_info(total_items, page, page_size),
    }
}

/* -------------------- Tests -------------------- */
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn clamp_and_meta() {
        let p = PageParams { page: 0, page_size: 9999 };
        let (page, size, offset, limit) = p.clamp(PaginationClamp::default());
        assert_eq!(page, 1);
        assert_eq!(size, 200);
        assert_eq!(offset, 0);
        assert_eq!(limit, 200);

        let info = make_page_info(1001, page, size);
        assert_eq!(info.total_pages, 6);
    }

    #[test]
    fn paged_wrap() {
        let p = PageParams::default();
        let (page, size, _, _) = p.clamp(PaginationClamp::default());
        let out = to_paged(vec![1,2,3], 3, page, size);
        assert_eq!(out.meta.total_items, 3);
        assert_eq!(out.data.len(), 3);
    }
}
