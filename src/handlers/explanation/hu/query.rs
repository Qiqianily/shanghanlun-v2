use std::collections::HashMap;

use axum::{debug_handler, extract::State};

use crate::{
    common::valid::{ValidPath, ValidQuery},
    handlers::{
        explanation::hu::model::{ExplanationHuModel, ExplanationTreatiseData},
        prescription::model::PrescriptionModel,
        relations::model::TreatisePrescriptionJoin,
        shared::model::PaginationParams,
        treatise::model::{QueryTreatiseIdParam, TreatiseModel},
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 按 id 来查询
#[debug_handler]
pub async fn query_explanation_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QueryTreatiseIdParam>,
) -> ApiResult<ApiResponse<ExplanationTreatiseData>> {
    let treatise_query_str = r#"
        SELECT id, uuid, chapter, section_number, content
        FROM treatise
        WHERE id = $1;
        "#;
    // 根据主键 ID 查询原文信息
    let treatise_model = sqlx::query_as::<_, TreatiseModel>(treatise_query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的条文暂时没查询到")))?;

    // 根据条文 id 来查询解析内容
    let explanation_query_str = r#"
        SELECT treatise_id, uuid, explanation, summary
        FROM explain_hu
        WHERE treatise_id = $1;
        "#;
    let explanation_model = sqlx::query_as::<_, ExplanationHuModel>(explanation_query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的条文解析暂时没查询到")))?;

    // 通过关联表来查方剂，可以没有，也可以有多个
    let relation_query_str = r#"
        SELECT p.id, p.uuid, p.name, p.ingredients, p.dosage, p.usage, p.function
        FROM prescription p
        JOIN treatise_prescription tp ON p.id = tp.prescription_id
        WHERE tp.treatise_id = $1
        ORDER BY p.id;
        "#;
    // 查询关联的方剂
    let prescriptions_model = sqlx::query_as::<_, PrescriptionModel>(relation_query_str)
        .bind(treatise_model.id)
        .fetch_all(db_pool)
        .await?;

    // 判断是否有方剂，如果没有则 prescriptions_opt 为 None
    let has_prescription = !prescriptions_model.is_empty();
    // 使用 then_some 构建 Option，如果有则 Some(prescriptions)，否则 None Rust >= 1.62
    let prescriptions_opt = has_prescription.then_some(prescriptions_model);
    // let prescriptions_opt = if has_prescription {
    //     Some(prescriptions_model)
    // } else {
    //     None
    // };
    // 构建返回的结果
    let result = ExplanationTreatiseData {
        has_prescription,
        treatise: treatise_model,
        prescriptions: prescriptions_opt,
        explanations: explanation_model,
    };
    Ok(ApiResponse::ok("success", Some(result)))
}

/// 分页查询
pub async fn query_explanations_pages_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(PaginationParams {
        keyword: _,
        pagination,
    }): ValidQuery<PaginationParams>,
) -> ApiResult<ApiResponse<Vec<ExplanationTreatiseData>>> {
    let page = pagination.page;
    let size = pagination.size;
    // 每页查询多少条数据
    let offset = ((page - 1) * size) as i64;
    let limit = size as i64;
    // 第一步：先分页查询原文
    // 1. 先分页查询原文
    let treatise_query_str = r#"
        SELECT id, uuid, chapter, section_number, content
        FROM treatise
        ORDER BY id
        LIMIT $1 OFFSET $2;
        "#;
    let treatises_model = sqlx::query_as::<_, TreatiseModel>(treatise_query_str)
        .bind(limit)
        .bind(offset)
        .fetch_all(db_pool)
        .await?;
    if treatises_model.is_empty() {
        return Ok(ApiResponse::ok("success", Some(vec![])));
    }
    // 第二步：提取原文 ID
    let treatise_ids: Vec<i32> = treatises_model.iter().map(|t| t.id).collect();

    // 第三步：根据条文 id 查询条文解析的 models
    let explanation_query_str = r#"
        SELECT treatise_id, uuid, explanation, summary
        FROM explain_hu
        WHERE treatise_id = ANY($1)
        ORDER BY treatise_id;
        "#;
    let explanations_model = sqlx::query_as::<_, ExplanationHuModel>(explanation_query_str)
        .bind(&treatise_ids)
        .fetch_all(db_pool)
        .await?;
    // 第四步：查询关联的方剂 返回的是关联表的信息和对应方剂的 model
    let prescription_query_str = r#"
        SELECT tp.treatise_id,
            p.id AS prescription_id,
			p.uuid, p.name, p.ingredients, p.dosage, p.usage, p.function
            FROM treatise_prescription tp
            INNER JOIN prescription p ON tp.prescription_id = p.id
            WHERE tp.treatise_id = ANY($1)
            ORDER BY tp.treatise_id, p.id;
        "#;
    let relations_vec = sqlx::query_as::<_, TreatisePrescriptionJoin>(prescription_query_str)
        .bind(&treatise_ids)
        .fetch_all(db_pool)
        .await?;
    // 按 treatise_id 分组方剂
    let mut prescriptions_by_treatise: HashMap<i32, Vec<PrescriptionModel>> = HashMap::new();
    for relation in relations_vec {
        let prescription = PrescriptionModel {
            id: relation.prescription_id,
            uuid: relation.uuid,
            name: relation.name,
            ingredients: relation.ingredients,
            dosage: relation.dosage,
            usage: relation.usage,
            function: relation.function,
        };
        prescriptions_by_treatise
            .entry(relation.treatise_id) // 检查 id 是否存在
            .or_default() // 不存在就插入一个空数组
            .push(prescription); // 如果 id 还在就不再创建新的 id 直接把 prescription 插入
    }

    // 组合结果
    let result: Vec<ExplanationTreatiseData> = treatises_model
        .into_iter()
        .map(|treatise| {
            let prescriptions = prescriptions_by_treatise
                .remove(&treatise.id)
                .unwrap_or_default();
            let explanation = explanations_model
                .iter()
                .find(|item| item.treatise_id == treatise.id)
                .unwrap()
                .clone();
            let has_prescription = !prescriptions.is_empty();
            // 返回组合的结果
            ExplanationTreatiseData {
                has_prescription,
                treatise,
                prescriptions: has_prescription.then_some(prescriptions),
                explanations: explanation,
            }
        })
        .collect();
    // let result: Vec<ExplanationTreatiseData> = treatise_ids
    //     .into_iter()
    //     .map(|treatise| {
    //         let prescriptions = prescriptions_by_treatise
    //             .remove(&treatise)
    //             .unwrap_or_default();
    //         // 获取索引
    //         let explanation = explanations_model
    //             .iter()
    //             .find(|item| item.treatise_id == treatise)
    //             .unwrap()
    //             .clone();
    //         let treatise_model = treatises_model
    //             .iter()
    //             .find(|item| item.id == treatise)
    //             .unwrap()
    //             .clone();
    //         if prescriptions.is_empty() {
    //             ExplanationTreatiseData {
    //                 has_prescription: false,
    //                 treatise: treatise_model,
    //                 prescriptions: None,
    //                 explanations: explanation,
    //             }
    //         } else {
    //             ExplanationTreatiseData {
    //                 has_prescription: true,
    //                 treatise: treatise_model,
    //                 prescriptions: Some(prescriptions),
    //                 explanations: explanation,
    //             }
    //         }
    //     })
    //     .collect();

    Ok(ApiResponse::ok("success", Some(result)))
}
