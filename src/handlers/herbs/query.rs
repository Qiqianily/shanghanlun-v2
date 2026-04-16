use axum::{debug_handler, extract::State};

use crate::{
    common::valid::{ValidPath, ValidQuery},
    handlers::{
        herbs::model::{QueryHerbsIdParam, ShenNongHerbsModel},
        shared::{
            model::QueryKeywordParam,
            pagination::{Pagination, PaginationResult},
        },
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 按 id 来查询
#[debug_handler]
pub async fn query_herb_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QueryHerbsIdParam>,
) -> ApiResult<ApiResponse<ShenNongHerbsModel>> {
    // 查询语句
    let query_str = r#"
        SELECT id, uuid, chapter, category, name, alias, content, other
        FROM shennong_herbs
        WHERE id = $1;
        "#;
    // 执行查询，如果没有查到就返回自定义错误
    let herb_model = sqlx::query_as::<_, ShenNongHerbsModel>(query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的本草暂时没查询到")))?;
    // 返回查询结果
    Ok(ApiResponse::ok("success", Some(herb_model)))
}

/// 通过药名或别名查询
#[debug_handler]
pub async fn query_herbs_by_name_or_alias_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<ShenNongHerbsModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::<ShenNongHerbsModel>::empty();
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 构造模糊查询的 SQL 语句
    let query_str = r#"
            SELECT id, uuid, chapter, category, name, alias, content, other
            FROM shennong_herbs
            WHERE name LIKE $1 OR alias LIKE $1
            ORDER BY id ASC;
            "#;
    let pattern = format!("%{}%", keyword);
    // 执行查询，如果没有查到就返回自定义错误
    let herbs_models = sqlx::query_as::<_, ShenNongHerbsModel>(query_str)
        .bind(&pattern)
        .fetch_all(db_pool)
        .await?;
    // 一共查到多少条数据
    let total = herbs_models.len() as u64;
    // 转换成分页的返回数据结构
    let results =
        PaginationResult::from_pagination_params(Pagination::initial_zero(), total, herbs_models);
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 通过关键字从内容中进行模糊查询
#[debug_handler]
pub async fn query_herbs_by_keyword_from_content_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<ShenNongHerbsModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::from_pagination_params(
            Pagination::initial_zero(),
            0,
            Vec::<ShenNongHerbsModel>::new(),
        );
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 构造模糊查询的 SQL 语句
    let query_str = r#"
            SELECT id, uuid, chapter, category, name, alias, content, other
            FROM shennong_herbs
            WHERE content LIKE $1
            ORDER BY id ASC;
            "#;
    let pattern = format!("%{}%", keyword);
    // 执行查询，如果没有查到就返回自定义错误
    let herbs_models = sqlx::query_as::<_, ShenNongHerbsModel>(query_str)
        .bind(&pattern)
        .fetch_all(db_pool)
        .await?;
    // 一共查到多少条数据
    let total = herbs_models.len() as u64;
    // 转换成分页的返回数据结构
    let results =
        PaginationResult::from_pagination_params(Pagination::initial_zero(), total, herbs_models);
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 通过分类查询
#[debug_handler]
pub async fn query_herbs_by_category_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<ShenNongHerbsModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::from_pagination_params(
            Pagination::initial_zero(),
            0,
            Vec::<ShenNongHerbsModel>::new(),
        );
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 构造模糊查询的 SQL 语句
    let query_str = r#"
            SELECT id, uuid, chapter, category, name, alias, content, other
            FROM shennong_herbs
            WHERE category = $1
            ORDER BY id ASC;
            "#;
    // 执行查询，如果没有查到就返回自定义错误
    let herbs_models = sqlx::query_as::<_, ShenNongHerbsModel>(query_str)
        .bind(&keyword)
        .fetch_all(db_pool)
        .await?;
    // 一共查到多少条数据
    let total = herbs_models.len() as u64;
    // 转换成分页的返回数据结构
    let results =
        PaginationResult::from_pagination_params(Pagination::initial_zero(), total, herbs_models);
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}
