use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TreatiseModel {
    pub id: i32,
    pub uuid: Uuid,
    pub chapter: String,
    pub section_number: String,
    pub content: Vec<String>,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

/// 定义查询条文的 id 范围
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QueryTreatiseIdParam {
    #[validate(range(min = 1, max = 398, message = "查询条文的 id 必须在 1-398 之间"))]
    pub id: i32,
}
