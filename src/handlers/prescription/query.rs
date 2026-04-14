use std::collections::HashSet;

use axum::{debug_handler, extract::State};

use crate::{
    common::valid::{ValidJson, ValidPath, ValidQuery},
    handlers::{
        prescription::model::{PrescriptionModel, QueryPrescriptionIdParam},
        shared::{
            model::{IngredientsParams, PaginationParams, QueryKeywordParam},
            pagination::{Pagination, PaginationResult},
        },
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 按 id 来查询
#[debug_handler]
pub async fn query_prescription_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QueryPrescriptionIdParam>,
) -> ApiResult<ApiResponse<PrescriptionModel>> {
    // 查询语句
    let query_str =
        "SELECT id, uuid, name,ingredients, dosage,usage,function FROM prescription WHERE id = $1";
    // 执行查询，如果没有查到就返回自定义错误
    let prescription_model = sqlx::query_as::<_, PrescriptionModel>(query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的方剂暂时没查到")))?;
    // 返回成功响应
    Ok(ApiResponse::ok("success", Some(prescription_model)))
}

/// 通过关键字在方剂名字中进行模糊查询
#[debug_handler]
pub async fn like_query_prescription_by_name_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<PrescriptionModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::from_pagination_params(
            Pagination::initial_zero(),
            0,
            Vec::<PrescriptionModel>::new(),
        );
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 根据关键字来查询
    let pattern = format!("%{}%", keyword);
    // 使用参数化查询，避免 SQL 注入
    let query_str = r#"SELECT id, uuid, name, ingredients, dosage, usage, function
            FROM prescription
            WHERE name LIKE $1
            ORDER BY id ASC"#;
    // 执行查询
    let prescription_models: Vec<PrescriptionModel> = sqlx::query_as(query_str)
        .bind(pattern)
        .fetch_all(db_pool)
        .await?;
    // 如果没有找到，返回空结果
    if prescription_models.is_empty() {
        let empty_result = PaginationResult::from_pagination_params(
            Pagination::initial_zero(),
            0,
            Vec::<PrescriptionModel>::new(),
        );
        return Ok(ApiResponse::ok(
            "根据你的方剂名，并没有找到这个方剂信息",
            Some(empty_result),
        ));
    }
    // 一共查到多少条数据
    let total = prescription_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        prescription_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 通过药物组成来查询
#[debug_handler]
pub async fn find_prescriptions_by_ingredients_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidJson(params): ValidJson<IngredientsParams>,
) -> ApiResult<ApiResponse<PaginationResult<PrescriptionModel>>> {
    // 1. 输入清洗：去重 + 去除空字符串
    let ingredients: Vec<String> = params
        .ingredients
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<HashSet<_>>() // 自动去重
        .into_iter()
        .collect();
    if ingredients.is_empty() {
        return Ok(ApiResponse::err("请至少提供一味有效药物"));
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

    // 如果没有找到数据
    if prescription_models.is_empty() {
        let empty_result = PaginationResult::from_pagination_params(
            Pagination::initial_zero(),
            0,
            Vec::<PrescriptionModel>::new(),
        );
        return Ok(ApiResponse::ok(
            "伤寒论中没有任何一张方剂中同时用到这几味药物！",
            Some(empty_result),
        ));
    }
    // 一共查到多少条数据
    let total = prescription_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        prescription_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 分页查询
#[debug_handler]
pub async fn query_prescription_by_function_pagination_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(PaginationParams {
        keyword,
        pagination,
    }): ValidQuery<PaginationParams>,
) -> ApiResult<ApiResponse<PaginationResult<PrescriptionModel>>> {
    // 每页查询多少条数据
    let limit = pagination.size as i64; // 每页条数
    // 查询第几页
    let offset = ((pagination.page - 1) * pagination.size) as i64;
    // 构造模糊匹配模式（若有关键词且非空）
    let like_pattern = keyword
        .as_ref()
        .filter(|k| !k.is_empty())
        .map(|k| format!("%{}%", k));

    // 1. 查询总记录数
    let total: i64 = if let Some(pattern) = &like_pattern {
        sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM prescription
            WHERE EXISTS (SELECT 1 FROM unnest(string_to_array("function", '，')) AS elem WHERE elem LIKE $1)"#,
        )
        .bind(pattern)
        .fetch_one(db_pool)
        .await?
    } else {
        sqlx::query_scalar("SELECT COUNT(*) FROM prescription")
            .fetch_one(db_pool)
            .await?
    };

    // 2. 查询分页数据
    let items = if let Some(pattern) = &like_pattern {
        sqlx::query_as::<_, PrescriptionModel>(
            r#"SELECT id, uuid, name, ingredients, dosage, usage, function
                 FROM prescription
                 WHERE EXISTS (SELECT 1 FROM unnest(string_to_array("function", '，')) AS elem WHERE elem LIKE $1)
                 ORDER BY id
                 LIMIT $2 OFFSET $3;"#,
        )
        .bind(pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(db_pool)
        .await?
    } else {
        sqlx::query_as::<_, PrescriptionModel>(
            "SELECT id, uuid, name, ingredients, dosage, usage, function
                 FROM prescription
                 ORDER BY id
                 LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(db_pool)
        .await?
    };
    // 构建返回的结果
    let result = PaginationResult::from_pagination_params(pagination, total as u64, items);
    // 返回数据
    Ok(ApiResponse::ok("success", Some(result)))
}
