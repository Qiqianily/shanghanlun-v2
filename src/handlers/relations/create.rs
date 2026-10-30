use axum::{Extension, debug_handler, extract::State};

use crate::{
    common::valid::ValidJson,
    handlers::relations::model::CreateRelationParams,
    middlewares::auth::{identity::Identity, principal::Principal},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 创建关系表数据
#[debug_handler]
pub async fn create_treatise_prescription_relation_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    Extension(principal): Extension<Principal>,
    ValidJson(payloads): ValidJson<CreateRelationParams>,
) -> ApiResult<ApiResponse<String>> {
    // 判断是否有修改权限
    let identity = principal.identity;
    if identity != Identity::Admin {
        return Err(ApiError::Unauthenticated("没有修改创建".into()));
    }
    let treatise_id = payloads.treatise_id;
    let prescription_id = payloads.prescription_id;
    let execute_str = r#"
        INSERT INTO treatise_prescription (treatise_id, prescription_id)
        VALUES ($1, $2)
        ON CONFLICT(treatise_id, prescription_id)
        DO NOTHING
    "#;
    let result = sqlx::query(execute_str)
        .bind(treatise_id)
        .bind(prescription_id)
        .execute(db_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    let message = if result.rows_affected() == 0 {
        format!("数据库中已经存在：{treatise_id}-{prescription_id} 的关系，无需再次创建！")
    } else {
        format!("插入成功：{treatise_id}-{prescription_id}")
    };

    Ok(ApiResponse::success(message))
}
