use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, FromRow, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShenNongHerbsModel {
    pub id: i32,
    pub uuid: Uuid,
    pub chapter: String,
    pub category: String,
    pub name: String,
    pub alias: Option<String>,
    pub content: String,
    pub other: Option<String>,
}

/// 定义查询本草的 id 范围
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QueryHerbsIdParam {
    #[validate(range(min = 1, max = 358, message = "查询本草的 id 必须在 1-358 之间"))]
    pub id: i32,
}
