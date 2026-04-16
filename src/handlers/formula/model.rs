use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::handlers::sub_formula::model::SubFormulaModel;

#[derive(Clone, Debug, PartialEq, FromRow, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormulaModel {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub source: Option<String>,
    pub chapter: Option<String>,
    pub section: Option<String>,
    pub category: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub dosage: Option<String>,
    pub usage: Option<String>,
    pub functions: Option<Vec<String>>,
    pub indications: Option<Vec<String>>,
    pub pathogenesis_analysis: Option<Vec<String>>,
    pub formula_analysis: Option<Vec<String>>,
    pub compatibility_feature: Option<String>,
    pub application_notes: Option<String>,
    pub song: Option<Vec<String>>,
}

/// 定义查询方剂的 id 范围
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QueryFormulaIdParam {
    #[validate(range(min = 1, max = 234, message = "查询方剂的 id 必须在 1-234 之间"))]
    pub id: i32,
}

/// 方剂和附方
#[derive(serde::Serialize)]
pub struct FormulaAndSubFormulaResponse {
    pub has_sub: bool,
    pub formula: FormulaModel,
    pub sub_formula: Option<Vec<SubFormulaModel>>,
}

/// 方剂的药物组成查询后返回的结果
#[derive(serde::Serialize, Debug)]
pub struct FormulaByIngredientsResponse {
    pub count: usize,
    pub formulas: Option<Vec<FormulaModel>>,
}

/// 定义方剂的功效最多 4 种
#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct FunctionsParams {
    #[validate(length(min = 1, max = 4, message = "方剂的功效不能太多4个以内"))]
    pub functions: Vec<String>,
}

/// 定义方剂的主治最多 4 种
#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct IndicationsParams {
    #[validate(length(min = 1, max = 4, message = "方剂的主治不能太多4个以内"))]
    pub indications: Vec<String>,
}
