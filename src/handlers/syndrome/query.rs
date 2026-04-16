use axum::{debug_handler, extract::State};

use crate::{
    common::valid::{ValidPath, ValidQuery},
    handlers::{
        shared::{
            model::QueryKeywordParam,
            pagination::{Pagination, PaginationResult},
        },
        syndrome::model::{QuerySyndromeIdParam, SyndromeModel},
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 按证候的 id 来查询
#[debug_handler]
pub async fn query_syndrome_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QuerySyndromeIdParam>,
) -> ApiResult<ApiResponse<SyndromeModel>> {
    // 查询语句
    let query_str = r#"
        SELECT id, uuid, chapter, section, title, alias, concept, key_points, manifestation, factors, pathogenesis, dev_transformation, disease_range, identification, treatment, prescription, drug_composition, prescription_analysis, treatment_points, modern_research
        FROM syndrome_tb
        WHERE id = $1;
        "#;
    // 执行查询，如果没有查到就返回自定义错误
    let syndrome_model = sqlx::query_as::<_, SyndromeModel>(query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的证候信息暂时没查到")))?;
    // 返回成功响应
    Ok(ApiResponse::ok("success", Some(syndrome_model)))
}

/// 按证候的名字和别名来查询
#[debug_handler]
pub async fn query_syndrome_by_title_or_alias_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<SyndromeModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::<SyndromeModel>::empty();
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 构造模糊查询的 SQL 语句
    let query_str = r#"
        SELECT id, uuid, chapter, section, title, alias, concept, key_points, manifestation, factors, pathogenesis, dev_transformation, disease_range, identification, treatment, prescription, drug_composition, prescription_analysis, treatment_points, modern_research
        FROM syndrome_tb
        WHERE title LIKE $1 OR alias LIKE $1
        ORDER BY id ASC;
        "#;
    // 根据证候的名字和别名来查询
    let pattern = format!("%{}%", keyword);
    // 执行查询，如果没有查到就返回自定义错误
    let syndromes_models = sqlx::query_as::<_, SyndromeModel>(query_str)
        .bind(&pattern)
        .fetch_all(db_pool)
        .await?;

    // 一共查到多少条数据
    let total = syndromes_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        syndromes_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 按照证候使用的方剂名字来查询
#[debug_handler]
pub async fn query_syndrome_by_prescription_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<SyndromeModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::<SyndromeModel>::empty();
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 构造模糊查询的 SQL 语句
    let query_str = r#"
        SELECT id, uuid, chapter, section, title, alias, concept, key_points, manifestation, factors, pathogenesis, dev_transformation, disease_range, identification, treatment, prescription, drug_composition, prescription_analysis, treatment_points, modern_research
        FROM syndrome_tb
        WHERE prescription LIKE $1
        ORDER BY id ASC;
        "#;
    // 根据证候的名字和别名来查询
    let pattern = format!("%{}%", keyword);
    // 执行查询，如果没有查到就返回自定义错误
    let syndromes_models = sqlx::query_as::<_, SyndromeModel>(query_str)
        .bind(&pattern)
        .fetch_all(db_pool)
        .await?;

    // 一共查到多少条数据
    let total = syndromes_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        syndromes_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 按证候的临床表现来查询
#[debug_handler]
pub async fn query_syndrome_by_manifestation_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<SyndromeModel>>> {
    // 需要构建这样的查询
    // SELECT * FROM syndrome
    // WHERE jsonb_typeof(manifestation) = 'array'
    // AND EXISTS (
    //     SELECT 1
    //     FROM jsonb_array_elements(manifestation) AS elem
    //     WHERE elem->>'主证' LIKE $1
    //        OR elem->>'症状' LIKE $1
    // )
    // ORDER BY id ASC;
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::<SyndromeModel>::empty();
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 构造模糊查询的 SQL 语句
    let query_str = r#"
        SELECT id, uuid, chapter, section, title, alias, concept, key_points, manifestation, factors, pathogenesis, dev_transformation, disease_range, identification, treatment, prescription, drug_composition, prescription_analysis, treatment_points, modern_research
        FROM syndrome_tb
        WHERE jsonb_typeof(manifestation) = 'array'
        AND EXISTS (
            SELECT 1
            FROM jsonb_array_elements(manifestation) AS elem
            WHERE elem->>'主证' LIKE $1
               OR elem->>'症状' LIKE $1
        )
        ORDER BY id ASC;
        "#;
    // 根据证候的名字和别名来查询
    let pattern = format!("%{}%", keyword);
    // 执行查询，如果没有查到就返回自定义错误
    let syndromes_models = sqlx::query_as::<_, SyndromeModel>(query_str)
        .bind(&pattern)
        .fetch_all(db_pool)
        .await?;

    // 一共查到多少条数据
    let total = syndromes_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        syndromes_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}
