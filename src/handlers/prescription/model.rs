use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrescriptionModel {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub ingredients: Vec<String>,
    pub dosage: Option<String>,
    pub usage: Option<String>,
    pub function: Option<String>,
}

/// 定义查询方剂的 id 范围
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QueryPrescriptionIdParam {
    #[validate(range(min = 1, max = 115, message = "查询方剂的 id 必须在 1-115 之间"))]
    pub id: i32,
}
