use crate::{handlers::formula, state::AppState};

/// 创建本草相关的路由，专门用来管理与本草相关的操作
pub fn create_formula_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/query/info/id/{id}",
            axum::routing::get(formula::query::query_formula_by_id_handler),
        )
        .route(
            "/query/infos/by/name",
            axum::routing::get(formula::query::query_formula_by_name_handler),
        )
        .route(
            "/query/infos/by/source",
            axum::routing::get(formula::query::query_formula_by_source_handler),
        )
        .route(
            "/query/infos/by/chapter",
            axum::routing::get(formula::query::query_formula_by_chapter_handler),
        )
        .route(
            "/query/infos/by/category",
            axum::routing::get(formula::query::query_formula_by_category_handler),
        )
        .route(
            "/query/infos/by/ingredients",
            axum::routing::post(formula::query::query_formula_by_ingredients_handler),
        )
        .route(
            "/query/infos/by/functions",
            axum::routing::post(formula::query::query_formula_by_functions_handler),
        )
        .route(
            "/query/infos/by/indications",
            axum::routing::post(formula::query::query_formula_by_indications_handler),
        )
}
