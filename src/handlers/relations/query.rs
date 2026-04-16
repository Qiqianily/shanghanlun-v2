use std::collections::{HashMap, HashSet};

use axum::{debug_handler, extract::State};

use crate::{
    common::valid::{ValidJson, ValidPath, ValidQuery},
    handlers::{
        prescription::model::PrescriptionModel,
        relations::model::{
            PrescriptionIngredientsToTreatise, PrescriptionNameParam, PrescriptionNameToTreatise,
            TreatiseJoin, TreatisePrescriptionJoin, TreatisePrescriptionRelation,
        },
        shared::model::{IngredientsParams, PaginationParams},
        treatise::model::{QueryTreatiseIdParam, TreatiseModel},
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 通过条文 ID 查询条文对应涉及到的方剂
#[debug_handler]
pub async fn query_treatise_prescriptions_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QueryTreatiseIdParam>,
) -> ApiResult<ApiResponse<TreatisePrescriptionRelation>> {
    let tratise_query_str = r#"
        SELECT id, uuid, chapter, section_number, content
        FROM treatise WHERE id = $1;
        "#;
    // 根据 ID 查询原文信息
    let treatise_model = sqlx::query_as::<_, TreatiseModel>(tratise_query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(format!("未找到 id = {} 的条文内容", params.id)))?;

    // 通过关联表来查方剂，可以没有，也可以有多个
    let relation_query_str = r#"
        SELECT p.id, p.uuid, p.name, p.ingredients, p.dosage, p.usage, p.function
        FROM prescription p
        JOIN treatise_prescription tp ON p.id = tp.prescription_id
        WHERE tp.treatise_id = $1
        ORDER BY p.id;
        "#;
    // 查询关联的方剂
    let prescriptions = sqlx::query_as::<_, PrescriptionModel>(relation_query_str)
        .bind(treatise_model.id)
        .fetch_all(db_pool)
        .await?;

    // 判断是否有方剂，如果没有则 prescriptions_opt 为 None
    let has_prescription = !prescriptions.is_empty();
    // 使用 then_some 构建 Option，如果有则 Some(prescriptions)，否则 None Rust >= 1.62
    let prescriptions_opt = has_prescription.then_some(prescriptions);
    // let prescriptions_opt = if has_prescription {
    //     Some(prescriptions)
    // } else {
    //     None
    // };
    // 构建返回的结果
    let result = TreatisePrescriptionRelation {
        has_prescription,
        treatise: treatise_model,
        prescriptions: prescriptions_opt,
    };
    // 返回结果
    Ok(ApiResponse::ok("success", Some(result)))
}

/// 通过方剂名称来查询涉及到的条文
#[debug_handler]
pub async fn find_treatises_by_prescription_name_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidJson(params): ValidJson<PrescriptionNameParam>,
) -> ApiResult<ApiResponse<PrescriptionNameToTreatise>> {
    // 1. 判空：如果方剂名称为空字符串，直接返回空结果
    if params.name.trim().is_empty() {
        // return Err(ApiError::Biz("方剂名称不能为空".to_string()));
        return Ok(ApiResponse::ok(
            "方剂名称不能为空",
            Some(PrescriptionNameToTreatise {
                total: 0,
                treatises: Some(Vec::new()),
            }),
        ));
    }
    // 2. 先根据方剂名称查询
    let prescription_query_str = r#"
        SELECT id, uuid, name,ingredients, dosage,usage,function
        FROM prescription
        WHERE name = $1
    "#;
    let prescription_opt = sqlx::query_as::<_, PrescriptionModel>(prescription_query_str)
        .bind(&params.name)
        .fetch_optional(db_pool)
        .await?;

    // 3. 若方剂不存在，直接返回空结果
    let Some(prescription) = prescription_opt else {
        return Ok(ApiResponse::ok(
            "根据你提供的方剂名没有找到对应的方剂",
            Some(PrescriptionNameToTreatise {
                total: 0,
                treatises: Some(Vec::new()),
            }),
        ));
    };
    // 4. 通过关系表来查询涉及到的原文
    let treatise_query_str = r#"
        SELECT t.id, t.uuid, t.chapter, t.section_number, t.content
                FROM treatise t
                JOIN treatise_prescription tp ON t.id = tp.treatise_id
                WHERE tp.prescription_id = $1
		ORDER BY t.id;
        "#;
    let treatises_model: Vec<TreatiseModel> = sqlx::query_as(treatise_query_str)
        .bind(prescription.id)
        .fetch_all(db_pool)
        .await?;
    // 5. 构建返回结果
    let result = PrescriptionNameToTreatise {
        total: treatises_model.len(),
        treatises: Some(treatises_model),
    };
    // 6. 返回数据
    Ok(ApiResponse::ok("success", Some(result)))
}

/// 通过方剂中的药物组成来查询涉及到的相关条文
///
#[debug_handler]
pub async fn find_treatises_by_prescription_ingredients_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidJson(params): ValidJson<IngredientsParams>,
) -> ApiResult<ApiResponse<PrescriptionIngredientsToTreatise>> {
    // 1. 输入清洗：去重 + 去除空字符串
    let ingredients: Vec<String> = params
        .ingredients
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<HashSet<_>>() // 自动去重
        .into_iter()
        .collect();
    if ingredients.is_empty() {
        let result = PrescriptionIngredientsToTreatise {
            count: 0,
            treatises: Some(vec![]),
        };
        return Ok(ApiResponse::ok("请至少提供一味有效药物", Some(result)));
    }
    // 构造 sql 查询语句
    let query_str = r#"SELECT id, uuid, name, ingredients, dosage, usage, function
        FROM prescription
        WHERE ingredients @> $1
        ORDER BY id"#;
    // 2. 使用参数化查询，彻底避免 SQL 注入  Vec<String> 自动映射为 text[]
    let prescription_models: Vec<PrescriptionModel> = sqlx::query_as(query_str)
        .bind(&ingredients)
        .fetch_all(db_pool)
        .await?;
    if prescription_models.is_empty() {
        // return Err(ApiError::Biz(String::from(
        //     "伤寒论中没有任何一张方剂中同时用到这几味药物！",
        // )));
        let result = PrescriptionIngredientsToTreatise {
            count: 0,
            treatises: Some(Vec::new()),
        };
        return Ok(ApiResponse::ok(
            "伤寒论中没有任何一张方剂中同时用到这几味药物！",
            Some(result),
        ));
    }
    // 3. 一条 JOIN 查询获取所有关联的条文（包含处方 ID 信息）
    let prescription_ids: Vec<i32> = prescription_models.iter().map(|p| p.id).collect();
    let query_treatises = r#"
           SELECT tp.prescription_id, t.id, t.uuid, t.chapter, t.section_number, t.content
           FROM treatise_prescription tp
           JOIN treatise t ON tp.treatise_id = t.id
           WHERE tp.prescription_id = ANY($1)
           ORDER BY tp.prescription_id, t.id;
       "#;
    let join_rows: Vec<TreatiseJoin> = sqlx::query_as(query_treatises)
        .bind(&prescription_ids)
        .fetch_all(db_pool)
        .await?;
    // 4. 按处方 ID 分组
    let mut map: HashMap<i32, Vec<TreatiseModel>> = HashMap::new();
    for row in join_rows {
        let treatise = TreatiseModel {
            id: row.id,
            uuid: row.uuid,
            chapter: row.chapter,
            section_number: row.section_number,
            content: row.content,
        };
        map.entry(row.prescription_id).or_default().push(treatise);
    }
    // 5. 按原始处方顺序构建 Vec<Vec<TreatiseModel>>
    let mut treatise_vec_vec = Vec::with_capacity(prescription_models.len());
    for prescription in &prescription_models {
        let treatises = map.remove(&prescription.id).unwrap_or_default();
        treatise_vec_vec.push(treatises);
    }
    let result = PrescriptionIngredientsToTreatise {
        count: treatise_vec_vec.len(),
        treatises: Some(treatise_vec_vec),
    };
    // 返回结果
    Ok(ApiResponse::ok("success", Some(result)))
}

/// 分页查询
pub async fn query_treatise_prescription_pages_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(PaginationParams {
        keyword: _,
        pagination,
    }): ValidQuery<PaginationParams>,
) -> ApiResult<ApiResponse<Vec<TreatisePrescriptionRelation>>> {
    let page = pagination.page;
    let size = pagination.size;
    // 每页查询多少条数据
    let offset = ((page - 1) * size) as i64;
    let limit = size as i64;
    // 1. 先分页查询原文
    let treatise_query_str = r#"
        SELECT id, uuid, chapter, section_number, content
        FROM treatise
        ORDER BY id
        LIMIT $1 OFFSET $2;
        "#;
    let treatises = sqlx::query_as::<_, TreatiseModel>(treatise_query_str)
        .bind(limit)
        .bind(offset)
        .fetch_all(db_pool)
        .await?;
    if treatises.is_empty() {
        return Ok(ApiResponse::ok("success", Some(vec![])));
    }
    // 提取原文 ID
    let treatise_ids: Vec<i32> = treatises.iter().map(|t| t.id).collect();
    // 2. 查询关联的方剂 返回的是关联表的信息和对应方剂的 model
    let prescription_query_str = r#"
        SELECT tp.treatise_id,
            p.id AS prescription_id,
			p.uuid, p.name, p.ingredients, p.dosage, p.usage, p.function
            FROM treatise_prescription tp
            INNER JOIN prescription p ON tp.prescription_id = p.id
            WHERE tp.treatise_id = ANY($1)
            ORDER BY tp.treatise_id, p.id;
        "#;
    let join_rows = sqlx::query_as::<_, TreatisePrescriptionJoin>(prescription_query_str)
        .bind(&treatise_ids)
        .fetch_all(db_pool)
        .await?;

    // 3. 按 treatise_id 分组构建 PrescriptionModel
    let mut prescriptions_map: HashMap<i32, Vec<PrescriptionModel>> = HashMap::new();
    for row in join_rows {
        let prescription = PrescriptionModel {
            id: row.prescription_id,
            uuid: row.uuid,
            name: row.name,
            ingredients: row.ingredients,
            dosage: row.dosage,
            usage: row.usage,
            function: row.function,
        };
        prescriptions_map
            .entry(row.treatise_id)
            .or_default()
            .push(prescription);
    }

    // 4. 组装最终结果（保持 treatise 原始顺序）
    let result: Vec<TreatisePrescriptionRelation> = treatises
        .into_iter()
        .map(|treatise| {
            let prescriptions = prescriptions_map.remove(&treatise.id).unwrap_or_default();
            if prescriptions.is_empty() {
                TreatisePrescriptionRelation {
                    has_prescription: false,
                    treatise,
                    prescriptions: None,
                }
            } else {
                TreatisePrescriptionRelation {
                    has_prescription: true,
                    treatise,
                    prescriptions: Some(prescriptions),
                }
            }
        })
        .collect();
    // 返回结果
    Ok(ApiResponse::ok("success", Some(result)))
}
