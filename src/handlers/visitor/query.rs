use axum::{debug_handler, extract::State};

use crate::{
    common::valid::ValidPath,
    handlers::visitor::model::{
        DailyStatsModel, VisitDaysParams, VisitDaysResult, VisitTodayParams,
    },
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::AppState,
};

/// 按起始时间和查询的天数来返回结果
#[debug_handler]
pub async fn query_visitors_by_days_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(VisitDaysParams { days, start }): ValidPath<VisitDaysParams>,
) -> ApiResult<ApiResponse<VisitDaysResult>> {
    tracing::info!("{:?}", days);
    let day_num = days.unwrap_or(7);
    tracing::info!("query_visitors_by_days_handler: day_num={}", day_num);
    let start_date = start
        .unwrap_or_else(|| chrono::Utc::now().date_naive() - chrono::Duration::days(day_num - 1));
    let query_str = r#"
        SELECT visit_date, uv_count
                FROM daily_stats
                WHERE visit_date >= $1
                ORDER BY visit_date ASC
                "#;
    let rows = sqlx::query_as::<_, DailyStatsModel>(query_str)
        .bind(start_date)
        .fetch_all(db_pool)
        .await?;
    // 总访问量
    let mut total = 0_i32;
    // 按日期填充结果
    let mut result = Vec::new();
    let mut idx = 0;
    for i in 0..day_num {
        let date = start_date + chrono::Duration::days(i);
        if idx < rows.len() && rows[idx].visit_date == date {
            result.push(rows[idx].clone());
            total += rows[idx].uv_count;
            idx += 1;
        } else {
            result.push(DailyStatsModel {
                visit_date: date,
                uv_count: 0,
            });
        }
    }

    let res = VisitDaysResult {
        stats: result,
        total_visits: total,
    };

    Ok(ApiResponse::success(res))
}

// 查询当天的访问数据
#[debug_handler]
pub async fn query_visitors_by_give_date_handler(
    State(AppState { db_pool, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    ValidPath(VisitTodayParams { today }): ValidPath<VisitTodayParams>,
) -> ApiResult<ApiResponse<DailyStatsModel>> {
    // 获取查询日期，默认为今天
    let start_date = today.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let query_str = r#"
        SELECT visit_date, uv_count
        FROM daily_stats
        WHERE visit_date = $1
        "#;
    let rows = sqlx::query_as::<_, DailyStatsModel>(query_str)
        .bind(start_date)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("你要查的访问信息还没有记录")))?;
    Ok(ApiResponse::success(rows))
}
