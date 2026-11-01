use axum::{Extension, extract::State};
use uuid::Uuid;

use crate::{
    common::valid::ValidJson,
    handlers::prescription::model::CreatePrescriptionParam,
    middlewares::auth::{identity::Identity, principal::Principal},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 创建方剂
#[axum::debug_handler]
pub async fn create_prescription_info_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    Extension(principal): Extension<Principal>,
    ValidJson(payloads): ValidJson<CreatePrescriptionParam>,
) -> ApiResult<ApiResponse<String>> {
    // 判断是否有修改权限
    let identity = principal.identity;
    if identity != Identity::Admin {
        return Err(ApiError::Unauthenticated("没有权限创建".into()));
    }
    // 构造 UPSERT SQL（PostgreSQL）如果名字冲突不会消耗自增 id 数
    let execute_str = r#"
        INSERT INTO prescription (name, ingredients, dosage, usage, function)
        SELECT $1, $2, $3, $4, $5
        WHERE NOT EXISTS (SELECT 1 FROM prescription WHERE name = $1)
        ON CONFLICT (name) DO NOTHING
        RETURNING id, uuid;
        "#;
    let name = payloads.name.clone();
    // name 冲突则 DO NOTHING，返回 None
    let inserted: Option<(i32, Uuid)> = sqlx::query_as::<_, (i32, Uuid)>(execute_str)
        .bind(&payloads.name)
        .bind(&payloads.ingredients)
        .bind(&payloads.dosage)
        .bind(&payloads.usage)
        .bind(&payloads.function)
        .fetch_optional(db_pool)
        .await?;
    match inserted {
        Some((id, uuid)) => Ok(ApiResponse::success(format!("{id}=={uuid}"))),
        // 冲突只在 name 上（uuid 是默认生成的），此时再查 id 给用户
        None => {
            let query_id_str = r#"
                SELECT id FROM prescription WHERE name = $1
                "#;
            let existing_id: Option<i32> = sqlx::query_scalar(query_id_str)
                .bind(&name)
                .fetch_optional(db_pool)
                .await?;
            Err(ApiError::Biz(match existing_id {
                Some(id) => format!("方剂名字已存在，id: {id}"),
                None => "方剂名字已存在".into(),
            }))
        }
    }
}
