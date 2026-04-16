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
