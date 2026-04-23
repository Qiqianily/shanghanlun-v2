use axum::{debug_handler, extract::State};
use chrono::Utc;

use crate::{
    response::{ApiResult, resp::ApiResponse},
    state::AppState,
    utils::get_vid::get_visitor_id,
};

#[debug_handler]
pub async fn insert_visit_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    headers: axum::http::HeaderMap,
) -> ApiResult<ApiResponse<()>> {
    // 64 位访客 id
    let visitor_id = get_visitor_id(&headers)?;
    // 2026-04-23
    let today_date = Utc::now().date_naive();
    // sql 语句
    let insert_sql_str = r#"
        INSERT INTO visits (visitor_id, visit_date)
                VALUES ($1, $2)
                ON CONFLICT (visitor_id, visit_date) DO NOTHING
    "#;
    let insert_result = sqlx::query(insert_sql_str)
        .bind(visitor_id)
        .bind(today_date)
        .execute(db_pool)
        .await?;
    // 如果插入了新记录，更新每日统计
    if insert_result.rows_affected() == 1 {
        sqlx::query(
            "INSERT INTO daily_stats (visit_date, uv_count) VALUES ($1, 1) ON CONFLICT (visit_date) DO UPDATE SET uv_count = daily_stats.uv_count + 1"
        )
        .bind(today_date)
        .execute(db_pool)
        .await?;
    }
    Ok(ApiResponse::success(()))
}
