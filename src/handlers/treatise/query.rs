use axum::{debug_handler, extract::State};

use crate::{
    common::valid::{ValidPath, ValidQuery},
    handlers::{
        shared::{
            model::{PaginationParams, QueryKeywordParam},
            pagination::{Pagination, PaginationResult},
        },
        treatise::model::{QueryTreatiseIdParam, TreatiseModel},
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 按 id 来查询
#[debug_handler]
pub async fn query_treatise_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QueryTreatiseIdParam>,
) -> ApiResult<ApiResponse<TreatiseModel>> {
    let query_str = "SELECT id, uuid, chapter, section_number, content FROM treatise WHERE id = $1";
    // 根据 ID 查询原文信息
    // let treatise: TreatiseModel = sqlx::query_as!(
    //     TreatiseModel,
    //     query_str,
    //     params.id
    // )
    // .fetch_optional(db_pool)
    // .await?
    // .ok_or_else(|| ApiError::Biz(String::from("你要查的条文暂时没查询到")))?;
    // let query_str = "SELECT * FROM treatise WHERE id = $1";

    let treatise_model = sqlx::query_as::<_, TreatiseModel>(query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的条文暂时没查询到")))?;

    // 返回查询结果
    Ok(ApiResponse::ok("success", Some(treatise_model)))
}

/// 通过关键字，模糊查询
#[debug_handler]
pub async fn query_treatise_by_keyword_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<TreatiseModel>>> {
    let query_str = r#"SELECT id, uuid, chapter, section_number, content
     FROM treatise
     WHERE EXISTS (
         SELECT 1 FROM unnest(content) AS elem
         WHERE elem LIKE $1
     )
     ORDER BY id;"#;
    // 根据关键字模糊查询
    let treatise_models = sqlx::query_as::<_, TreatiseModel>(query_str)
        .bind(format!("%{}%", keyword))
        .fetch_all(db_pool)
        .await?;
    // 一共查到多少条数据
    let total = treatise_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        treatise_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 分页查询
#[debug_handler]
pub async fn query_treatise_pagination_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(PaginationParams {
        keyword,
        pagination,
    }): ValidQuery<PaginationParams>,
) -> ApiResult<ApiResponse<PaginationResult<TreatiseModel>>> {
    // 分页参数
    let limit = pagination.size as i64; // 每页条数
    let offset = ((pagination.page - 1) * pagination.size) as i64; // 偏移量

    // 构造模糊匹配模式（若有关键词且非空）
    let like_pattern = keyword
        .as_ref()
        .filter(|k| !k.is_empty())
        .map(|k| format!("%{}%", k));

    // 1. 查询总记录数
    let total: i64 = if let Some(pattern) = &like_pattern {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM treatise
                 WHERE EXISTS (SELECT 1 FROM unnest(content) AS elem WHERE elem LIKE $1)",
        )
        .bind(pattern)
        .fetch_one(db_pool)
        .await?
    } else {
        sqlx::query_scalar("SELECT COUNT(*) FROM treatise")
            .fetch_one(db_pool)
            .await?
    };

    // 2. 查询分页数据
    let items = if let Some(pattern) = &like_pattern {
        sqlx::query_as::<_, TreatiseModel>(
            "SELECT id, uuid, chapter, section_number, content
                 FROM treatise
                 WHERE EXISTS (SELECT 1 FROM unnest(content) AS elem WHERE elem LIKE $1)
                 ORDER BY id
                 LIMIT $2 OFFSET $3",
        )
        .bind(pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(db_pool)
        .await?
    } else {
        sqlx::query_as::<_, TreatiseModel>(
            "SELECT id, uuid, chapter, section_number, content
                 FROM treatise
                 ORDER BY id
                 LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(db_pool)
        .await?
    };

    // 构造分页结果
    let result = PaginationResult::from_pagination_params(pagination, total as u64, items);
    Ok(ApiResponse::ok("success", Some(result)))
}
