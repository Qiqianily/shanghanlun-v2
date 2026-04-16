use crate::{handlers::sub_formula, state::AppState};

/// 创建附方相关的路由，专门用来管理与附方相关的操作
pub fn create_sub_formula_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/query/info/id/{id}",
            axum::routing::get(sub_formula::query::query_sub_formula_by_id_handler),
        )
        .route(
            "/query/infos/by/name",
            axum::routing::get(sub_formula::query::query_sub_formula_by_name_handler),
        )
        .route(
            "/query/infos/by/source",
            axum::routing::get(sub_formula::query::query_sub_formula_by_source_handler),
        )
        .route(
            "/query/infos/by/ingredients",
            axum::routing::post(sub_formula::query::query_sub_formula_by_ingredients_handler),
        )
        .route(
            "/query/infos/by/functions",
            axum::routing::post(sub_formula::query::query_sub_formula_by_functions_handler),
        )
        .route(
            "/query/infos/by/indications",
            axum::routing::post(sub_formula::query::query_sub_formula_by_indications_handler),
        )
}
