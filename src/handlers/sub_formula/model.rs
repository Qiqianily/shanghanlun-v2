use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, FromRow, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubFormulaModel {
    pub id: i32,
    pub uuid: Uuid,
    pub formula_id: i32,
    pub name: String,
    pub source: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub dosage: Option<String>,
    pub usage: Option<String>,
    pub functions: Option<Vec<String>>,
    pub indications: Option<Vec<String>>,
}

/// 定义查询方剂的 id 范围
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QuerySubFormulaIdParam {
    #[validate(range(min = 1, max = 182, message = "查询方剂的 id 必须在 1-182 之间"))]
    pub id: i32,
}

// 附方返回的多个结果
#[derive(serde::Serialize, Debug)]
pub struct SubFormulaByIngredientsResponse {
    pub count: usize,
    pub sub_formulas: Option<Vec<SubFormulaModel>>,
}
