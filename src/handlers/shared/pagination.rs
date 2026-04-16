use crate::common::serde::deserialize_string_or_number;
const DEFAULT_PAGE: usize = 1;
const DEFAULT_SIZE: usize = 20;
// 获取默认数据
fn default_page() -> usize {
    DEFAULT_PAGE
}
fn default_limit() -> usize {
    DEFAULT_SIZE
}
#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, validator::Validate)]
pub struct Pagination {
    #[validate(range(min = 1, message = "页码必须是大于 0 的正数"))]
    #[serde(
        default = "default_page",
        deserialize_with = "deserialize_string_or_number"
    )]
    pub page: usize,
    #[validate(range(min = 1, max = 100, message = "分页大小必须在 1 到 100 之间"))]
    #[serde(
        default = "default_limit",
        deserialize_with = "deserialize_string_or_number"
    )]
    pub size: usize,
}

impl Pagination {
    pub fn initial_zero() -> Self {
        Self { page: 0, size: 0 }
    }
}

// 分页查询返回的数据结构体
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    pub page: usize,
    pub limit: usize,
    pub total: u64,
    pub items: Vec<T>,
}

impl<T> PaginationResult<T> {
    pub fn new(page: usize, limit: usize, total: u64, items: Vec<T>) -> Self {
        Self {
            page,
            limit,
            total,
            items,
        }
    }

    // 从传入的分页查询参数中进行转换
    pub fn from_pagination_params(pagination: Pagination, total: u64, item: Vec<T>) -> Self {
        Self::new(pagination.page, pagination.size, total, item)
    }

    // 返回一个空的分页结果
    pub fn empty() -> Self {
        Self::new(0, 0, 0, Vec::new())
    }
}
