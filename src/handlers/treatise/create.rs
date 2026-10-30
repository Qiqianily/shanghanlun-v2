use axum::{Extension, debug_handler, extract::State};
use uuid::Uuid;

use crate::{
    common::valid::ValidJson,
    handlers::treatise::model::CreateTreatiseRequest,
    middlewares::auth::{identity::Identity, principal::Principal},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 支持创建新条文和更新已有条文
#[debug_handler]
pub async fn create_or_update_treatise_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    Extension(principal): Extension<Principal>,
    ValidJson(payloads): ValidJson<CreateTreatiseRequest>,
) -> ApiResult<ApiResponse<String>> {
    // 判断是否有修改权限
    let identity = principal.identity;
    if identity != Identity::Admin {
        return Err(ApiError::Unauthenticated("没有修改权限".into()));
    }
    // 查看章节号是否被占用
    let existing: Option<i32> =
        sqlx::query_scalar("SELECT id FROM treatise WHERE chapter = $1 AND section_number = $2")
            .bind(&payloads.chapter)
            .bind(&payloads.section_number)
            .fetch_optional(db_pool)
            .await?;
    // 如果章节号已被占用，且不是当前要更新的条文，返回错误
    if let Some(existing_id) = existing
        && existing_id != payloads.id
    {
        return Err(ApiError::ChapterSectionOccupied(
            "章节号已被其他条文占用".into(),
        ));
    }

    // 构造 UPSERT SQL（PostgreSQL）
    let execute_str = r#"
        INSERT INTO treatise (id, chapter, section_number, content)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id)
        DO UPDATE SET
            chapter = EXCLUDED.chapter,
            section_number = EXCLUDED.section_number,
            content = EXCLUDED.content
        RETURNING uuid;
    "#;
    // 执行 SQL 更新到数据库
    let record_uuid = sqlx::query_scalar::<_, Uuid>(execute_str)
        .bind(payloads.id)
        .bind(payloads.chapter)
        .bind(payloads.section_number)
        .bind(payloads.content)
        .fetch_one(db_pool)
        .await?;

    Ok(ApiResponse::success(record_uuid.to_string()))
}
