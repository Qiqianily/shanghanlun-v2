use crate::{handlers::explanation, middlewares::auth::layer::get_auth_layer, state::AppState};

/// 创建原文相关的路由，专门用来管理与原文相关的操作
pub fn create_explanation_hu_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/hu/create/explanation",
            axum::routing::post(explanation::hu::create::create_explanation_hu_handler),
        )
        .route_layer(get_auth_layer()) // 这里加上权限认证
        .route(
            "/hu/query/info/id/{id}",
            axum::routing::get(explanation::hu::query::query_explanation_by_id_handler),
        )
        .route(
            "/hu/query/explanations/paginate",
            axum::routing::post(explanation::hu::query::query_explanations_pages_handler),
        )
}
