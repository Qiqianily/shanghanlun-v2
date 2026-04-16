use std::collections::HashSet;

use axum::{debug_handler, extract::State};

use crate::{
    common::valid::{ValidJson, ValidPath, ValidQuery},
    handlers::{
        formula::model::{FunctionsParams, IndicationsParams},
        shared::{
            model::{IngredientsParams, QueryKeywordParam},
            pagination::{Pagination, PaginationResult},
        },
        sub_formula::model::{
            QuerySubFormulaIdParam, SubFormulaByIngredientsResponse, SubFormulaModel,
        },
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 按 id 来查询
#[debug_handler]
pub async fn query_sub_formula_by_id_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(params): ValidPath<QuerySubFormulaIdParam>,
) -> ApiResult<ApiResponse<SubFormulaModel>> {
    // 查询语句
    let sub_formula_query_str = r#"
        SELECT id, uuid, formula_id, name, source, ingredients, dosage, usage, functions, indications
        FROM sub_formula
        WHERE id = $1;
    "#;
    // 根据 ID 查询附方信息
    let sub_formula_model = sqlx::query_as::<_, SubFormulaModel>(sub_formula_query_str)
        .bind(params.id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的附方暂时没查询到")))?;
    // 返回查询结果
    Ok(ApiResponse::ok("success", Some(sub_formula_model)))
}

/// 按方剂名字来查询
#[debug_handler]
pub async fn query_sub_formula_by_name_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<SubFormulaModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::<SubFormulaModel>::empty();
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 查询语句
    let sub_formula_query_str = r#"
        SELECT id, uuid, formula_id, name, source, ingredients, dosage, usage, functions, indications
        FROM sub_formula
        WHERE name LIKE $1
        ORDER BY id;
    "#;
    let pattern = format!("%{}%", keyword);
    // 根据方剂名字来查询
    let sub_formula_models = sqlx::query_as::<_, SubFormulaModel>(sub_formula_query_str)
        .bind(pattern)
        .fetch_all(db_pool)
        .await?;
    // 一共查到多少条数据
    let total = sub_formula_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        sub_formula_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 按方剂来源来查询
#[debug_handler]
pub async fn query_sub_formula_by_source_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidQuery(QueryKeywordParam { keyword }): ValidQuery<QueryKeywordParam>,
) -> ApiResult<ApiResponse<PaginationResult<SubFormulaModel>>> {
    // 如果关键字为空白，直接返回空结果（或根据业务需求返回错误）
    if keyword.trim().is_empty() {
        let empty_result = PaginationResult::<SubFormulaModel>::empty();
        return Ok(ApiResponse::ok(
            "未提供有效的搜索关键字",
            Some(empty_result),
        ));
    }
    // 查询语句
    let sub_formula_query_str = r#"
        SELECT id, uuid, formula_id, name, source, ingredients, dosage, usage, functions, indications
        FROM sub_formula
        WHERE source = $1
        ORDER BY id;
    "#;
    let source = format!("《{}》", keyword);
    // 根据方剂来源出处查询
    let sub_formula_models = sqlx::query_as::<_, SubFormulaModel>(sub_formula_query_str)
        .bind(&source)
        .fetch_all(db_pool)
        .await?;
    // 一共查到多少条数据
    let total = sub_formula_models.len() as u64;
    // 转换成分页的返回数据结构
    let results = PaginationResult::from_pagination_params(
        Pagination::initial_zero(),
        total,
        sub_formula_models,
    );
    // 把结果返回
    Ok(ApiResponse::ok("success", Some(results)))
}

/// 按照附方的药物组成来查询
#[debug_handler]
pub async fn query_sub_formula_by_ingredients_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidJson(params): ValidJson<IngredientsParams>,
) -> ApiResult<ApiResponse<SubFormulaByIngredientsResponse>> {
    // 1. 输入清洗：去重 + 去除空字符串
    let ingredients: Vec<String> = params
        .ingredients
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<HashSet<_>>() // 自动去重
        .into_iter()
        .collect();
    if ingredients.is_empty() {
        let empty_result = SubFormulaByIngredientsResponse {
            count: 0,
            sub_formulas: None,
        };
        return Ok(ApiResponse::ok(
            "请至少提供一味有效药物",
            Some(empty_result),
        ));
    }
    let sub_formula_query_str = r#"
        SELECT id, uuid, formula_id, name, source, ingredients, dosage, usage, functions, indications
        FROM sub_formula
        WHERE ingredients @> $1
        ORDER BY id ASC;
    "#;
    // 查询
    // SELECT * FROM sub_formula WHERE ingredients @> ARRAY ['桂枝','芍药'] ORDER BY id ASC
    let sub_formulas_models = sqlx::query_as::<_, SubFormulaModel>(sub_formula_query_str)
        .bind(&ingredients)
        .fetch_all(db_pool)
        .await?;
    // 判断是不是空
    let count = sub_formulas_models.len();
    let has_sub_formulas = !sub_formulas_models.is_empty();
    let sub_formulas_opt = has_sub_formulas.then_some(sub_formulas_models);
    // let sub_formulas_opt = if has_sub_formulas {
    //     Some(sub_formulas_models)
    // } else {
    //     None
    // };
    // 构建返回结果
    let result = SubFormulaByIngredientsResponse {
        count,
        sub_formulas: sub_formulas_opt,
    };
    Ok(ApiResponse::ok("success", Some(result)))
}

/// 按照方剂的功效来查询
#[debug_handler]
pub async fn query_sub_formula_by_functions_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidJson(params): ValidJson<FunctionsParams>,
) -> ApiResult<ApiResponse<SubFormulaByIngredientsResponse>> {
    // 1. 输入清洗：去重 + 去除空字符串
    let functions: Vec<String> = params
        .functions
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<HashSet<_>>() // 自动去重
        .into_iter()
        .collect();
    if functions.is_empty() {
        let empty_result = SubFormulaByIngredientsResponse {
            count: 0,
            sub_formulas: None,
        };
        return Ok(ApiResponse::ok(
            "请至少提供一种功效来进行查询",
            Some(empty_result),
        ));
    }
    // 生成查询语句
    let sub_formula_query_str = r#"
        SELECT id, uuid, formula_id, name, source, ingredients, dosage, usage, functions, indications
        FROM sub_formula
        WHERE functions @> $1
        ORDER BY id ASC;
    "#;

    // 查询
    // SELECT * FROM formula WHERE functions @> ARRAY ['温中补虚','和里缓急'] ORDER BY id ASC
    let sub_formulas_models = sqlx::query_as::<_, SubFormulaModel>(sub_formula_query_str)
        .bind(&functions)
        .fetch_all(db_pool)
        .await?;
    // 判断是不是空
    let count = sub_formulas_models.len();
    let has_sub_formulas = !sub_formulas_models.is_empty();
    let sub_formulas_opt = has_sub_formulas.then_some(sub_formulas_models);
    // let sub_formulas_opt = if has_sub_formulas {
    //     Some(sub_formulas_models)
    // } else {
    //     None
    // };
    // 构建返回结果
    let result = SubFormulaByIngredientsResponse {
        count,
        sub_formulas: sub_formulas_opt,
    };
    Ok(ApiResponse::ok("success", Some(result)))
}

/// 按照方剂的治证来查询
#[debug_handler]
pub async fn query_sub_formula_by_indications_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidJson(params): ValidJson<IndicationsParams>,
) -> ApiResult<ApiResponse<SubFormulaByIngredientsResponse>> {
    // 生成查询语句 any 生成的是 OR all 生成的是 AND
    // SELECT * FROM "sub_formula" WHERE (EXISTS (SELECT 1 FROM unnest(indications)
    // AS ing WHERE ing LIKE '%外感风寒%')) AND
    // (EXISTS (SELECT 1 FROM unnest(indications)
    // AS ing WHERE ing LIKE '%身体疼重%'))
    // 1. 输入清洗：去重 + 去除空字符串
    let indications: Vec<String> = params
        .indications
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<HashSet<_>>() // 自动去重
        .into_iter()
        .collect();
    if indications.is_empty() {
        let empty_result = SubFormulaByIngredientsResponse {
            count: 0,
            sub_formulas: None,
        };
        return Ok(ApiResponse::ok(
            "请至少提供一种主治的名称来进行查询",
            Some(empty_result),
        ));
    }
    // 构造 SQL：基础查询 + 动态 EXISTS 子句
    let mut sub_formula_query_str = String::from("SELECT id, uuid, formula_id, name, source, ingredients, dosage, usage, functions, indications
    FROM sub_formula
    WHERE ");
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    for (i, kw) in indications.iter().enumerate() {
        // 每个子句使用不同的参数占位符 $1, $2, ...
        let placeholder = format!("${}", i + 1);
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM unnest(indications) AS ing WHERE ing LIKE {})",
            placeholder
        ));
        params.push(format!("%{}%", kw)); // 构造模糊匹配模式
    }
    // 使用 AND 连接多个 EXISTS 子句
    sub_formula_query_str.push_str(&conditions.join(" AND "));
    // 添加排序
    sub_formula_query_str.push_str(" ORDER BY id ASC;");
    // tracing::info!("query str:{}", formula_query_str);
    let mut query = sqlx::query_as::<_, SubFormulaModel>(&sub_formula_query_str);
    // 绑定参数
    for param in params {
        query = query.bind(param);
    }
    // 查询
    let sub_formulas_models = query.fetch_all(db_pool).await?;
    // 判断是不是空
    let count = sub_formulas_models.len();
    let has_sub_formulas = !sub_formulas_models.is_empty();
    let sub_formulas_opt = has_sub_formulas.then_some(sub_formulas_models);
    // let sub_formulas_opt = if has_sub_formulas {
    //     Some(sub_formulas_models)
    // } else {
    //     None
    // };
    // 构建返回结果
    let result = SubFormulaByIngredientsResponse {
        count,
        sub_formulas: sub_formulas_opt,
    };
    Ok(ApiResponse::ok("success", Some(result)))
}
