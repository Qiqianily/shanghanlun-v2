use crate::{handlers::prescription, state::AppState};

/// 创建原文相关的路由，专门用来管理与原文相关的操作
pub fn create_prescription_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/query/info/id/{id}",
            axum::routing::get(prescription::query::query_prescription_by_id_handler),
        )
        .route(
            "/query/pages/infos/by/function",
            axum::routing::post(
                prescription::query::query_prescription_by_function_pagination_handler,
            ),
        )
        .route(
            "/query/infos/by/name",
            axum::routing::get(prescription::query::like_query_prescription_by_name_handler),
        )
        .route(
            "/query/infos/by/ingredients",
            axum::routing::post(prescription::query::find_prescriptions_by_ingredients_handler),
        )
}
