use crate::{
    common::valid::ValidJson,
    handlers::prescription::model::CreatePrescriptionParam,
    middlewares::auth::{identity::Identity, principal::Principal},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};
use axum::{Extension, extract::State};

/// 更新处方信息
#[axum::debug_handler]
pub async fn update_prescription_info_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    Extension(principal): Extension<Principal>,
    ValidJson(payloads): ValidJson<CreatePrescriptionParam>,
) -> ApiResult<ApiResponse<String>> {
    // 判断是否有修改权限
    let identity = principal.identity;
    if identity != Identity::Admin {
        return Err(ApiError::Unauthenticated("没有权限更新".into()));
    }
    // 构造 UPSERT SQL（PostgreSQL）
    let execute_str = r#"
        INSERT INTO prescription (name, ingredients, dosage, usage, function)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (name)
        DO UPDATE SET
            ingredients = EXCLUDED.ingredients,
            dosage = EXCLUDED.dosage,
            usage = EXCLUDED.usage,
            function = EXCLUDED.function
        RETURNING name
        "#;
    // 执行 UPSERT 并获取返回的 name
    let record_name = sqlx::query_scalar::<_, String>(execute_str)
        .bind(&payloads.name)
        .bind(&payloads.ingredients)
        .bind(&payloads.dosage)
        .bind(&payloads.usage)
        .bind(&payloads.function)
        .fetch_one(db_pool)
        .await?;
    // 返回成功响应
    Ok(ApiResponse::success(record_name))
}
