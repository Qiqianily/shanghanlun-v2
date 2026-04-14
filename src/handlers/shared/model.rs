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

/// 定义药物名字组成的参数，最多 20 味
#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct IngredientsParams {
    #[validate(length(min = 1, max = 20, message = "药物组成数量不能太多20个以内"))]
    pub ingredients: Vec<String>,
}
