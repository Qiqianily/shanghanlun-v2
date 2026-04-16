use axum::{Extension, debug_handler, extract::State};
use sqlx::types::JsonValue;
use uuid::Uuid;

use crate::{
    common::valid::ValidJson,
    handlers::explanation::hu::model::CreateExplanationHuRequest,
    middlewares::auth::{identity::Identity, principal::Principal},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

#[debug_handler]
pub async fn create_explanation_hu_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    Extension(principal): Extension<Principal>,
    ValidJson(payloads): ValidJson<CreateExplanationHuRequest>,
) -> ApiResult<ApiResponse<String>> {
    // 判断是否有修改权限
    let identity = principal.identity;
    if identity != Identity::Admin {
        return Err(ApiError::Unauthenticated("没有修改权限".into()));
    }

    // 将 explanation 序列化为 JSON
    let explanation_json: JsonValue = serde_json::to_value(&payloads.explanation)?;

    // 构造 UPSERT SQL（PostgreSQL）
    let sql = r#"
            INSERT INTO explain_hu (treatise_id, explanation, summary)
            VALUES ($1, $2, $3)
            ON CONFLICT (treatise_id)
            DO UPDATE SET
                explanation = EXCLUDED.explanation,
                summary = EXCLUDED.summary
            RETURNING uuid;
        "#;
    // 更新到数据库
    let record_uuid = sqlx::query_scalar::<_, Uuid>(sql)
        .bind(payloads.treatise_id)
        .bind(&explanation_json)
        .bind(&payloads.summary)
        .fetch_one(db_pool)
        .await?;
    // .ok_or(ApiError::Biz("Upsert returned no row".into()))?;

    Ok(ApiResponse::success(record_uuid.to_string()))
}
