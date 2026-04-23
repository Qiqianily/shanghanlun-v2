use chrono::NaiveDate;
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VisitsModel {
    pub id: i32,
    pub visitor_id: String,
    pub visit_date: NaiveDate,
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DailyStatsModel {
    pub visit_date: NaiveDate,
    pub uv_count: i32,
}

/// 按天数查询访问量
#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "snake_case")]
pub struct VisitDaysParams {
    // 查询的天数，必须大于 1 天
    #[validate(range(min = 1))]
    pub days: Option<i64>,
    pub start: Option<NaiveDate>,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct VisitDaysResult {
    pub stats: Vec<DailyStatsModel>,
    pub total_visits: i32,
}

/// 查询当天的访问量
#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "snake_case")]
pub struct VisitTodayParams {
    pub today: Option<NaiveDate>,
}
