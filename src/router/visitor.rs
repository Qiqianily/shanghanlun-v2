use crate::{handlers::visitor, state::AppState};

/// 创建原文相关的路由，专门用来管理与原文相关的操作
pub fn create_visitor_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/create/info",
            axum::routing::post(visitor::insert::insert_visit_handler),
        )
        .route(
            "/query/info/by/days/{days}/{start}",
            axum::routing::get(visitor::query::query_visitors_by_days_handler),
        )
        .route(
            "/query/info/by/today/{today}",
            axum::routing::get(visitor::query::query_visitors_by_give_date_handler),
        )
}
