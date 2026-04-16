use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::handlers::{prescription::model::PrescriptionModel, treatise::model::TreatiseModel};

#[derive(Clone, Debug, PartialEq, FromRow, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExplanationHuModel {
    pub treatise_id: i32,
    pub uuid: Uuid,
    pub explanation: serde_json::Value,
    pub summary: Vec<String>,
}

/// 返回的数据结构
#[derive(serde::Serialize)]
pub struct ExplanationTreatiseData {
    pub has_prescription: bool,
    pub treatise: TreatiseModel,
    pub prescriptions: Option<Vec<PrescriptionModel>>,
    pub explanations: ExplanationHuModel,
}

// 辅助类型：单个解释项（一个键值对的对象）
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ExplanationItem {
    #[serde(flatten)]
    pub entry: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateExplanationHuRequest {
    #[validate(range(min = 1, max = 398, message = "条文的 id 必须在 1-398 之间"))]
    pub treatise_id: i32,
    pub explanation: Vec<std::collections::HashMap<String, String>>,
    pub summary: Vec<String>,
}
