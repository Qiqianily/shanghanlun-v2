use crate::handlers::shared::pagination::Pagination;
use validator::Validate;
/// 模糊查询关键字
#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct QueryKeywordParam {
    pub keyword: String,
}

// 分页查询相关信息
#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct PaginationParams {
    pub keyword: Option<String>,
    #[validate(nested)] // validate the inner data
    #[serde(flatten)]
    pub pagination: Pagination,
}
