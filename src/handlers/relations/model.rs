use uuid::Uuid;

use crate::handlers::{prescription::model::PrescriptionModel, treatise::model::TreatiseModel};

/// 分页返回的数据结构
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TreatisePrescriptionRelation {
    pub has_prescription: bool,
    pub treatise: TreatiseModel,
    pub prescriptions: Option<Vec<PrescriptionModel>>,
}

/// 方剂名称
#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct PrescriptionNameParam {
    pub name: String,
}

/// 通过处方名称查询关联的条文
#[derive(serde::Serialize, Debug)]
pub struct PrescriptionNameToTreatise {
    pub total: usize,
    pub treatises: Option<Vec<TreatiseModel>>,
}
/// 通过药物组成来查询关联的条文
#[derive(serde::Serialize, Debug)]
pub struct PrescriptionIngredientsToTreatise {
    pub count: usize,
    pub treatises: Option<Vec<Vec<TreatiseModel>>>,
}

// 临时结构用于接收条文的 JOIN 结果
#[derive(sqlx::FromRow)]
pub struct TreatiseJoin {
    pub prescription_id: i32,
    pub id: i32,
    pub uuid: Uuid,
    pub chapter: String,
    pub section_number: String,
    pub content: Vec<String>,
}
/// 临时用来接收方剂相关信息
#[derive(sqlx::FromRow)]
pub struct TreatisePrescriptionJoin {
    pub treatise_id: i32,
    pub prescription_id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub ingredients: Vec<String>,
    pub dosage: Option<String>,
    pub usage: Option<String>,
    pub function: Option<String>,
}
