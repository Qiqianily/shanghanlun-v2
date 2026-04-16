use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, FromRow, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SyndromeModel {
    pub id: i32,
    pub uuid: Uuid,
    pub chapter: String,
    pub section: String,
    pub title: String,
    pub alias: String,
    pub concept: String,
    pub key_points: String,
    pub manifestation: serde_json::Value,
    pub factors: Option<Vec<String>>,
    pub pathogenesis: String,
    pub dev_transformation: Option<Vec<String>>,
    pub disease_range: Option<Vec<String>>,
    pub identification: serde_json::Value,
    pub treatment: String,
    pub prescription: String,
    pub drug_composition: Option<Vec<String>>,
    pub prescription_analysis: String,
    pub treatment_points: String,
    pub modern_research: Option<String>,
}

#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct QuerySyndromeIdParam {
    #[validate(range(min = 1, max = 150, message = "查询证候的 id 必须在 1-150 之间"))]
    pub id: i32,
}
