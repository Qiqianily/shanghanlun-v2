use crate::{handlers::herbs, state::AppState};

/// 创建本草相关的路由，专门用来管理与本草相关的操作
pub fn create_treatise_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/query/info/id/{id}",
            axum::routing::get(herbs::query::query_herb_by_id_handler),
        )
        .route(
            "/query/infos/by/name",
            axum::routing::get(herbs::query::query_herbs_by_name_or_alias_handler),
        )
        .route(
            "/query/infos/from/content",
            axum::routing::get(herbs::query::query_herbs_by_keyword_from_content_handler),
        )
        .route(
            "/query/infos/by/category",
            axum::routing::get(herbs::query::query_herbs_by_category_handler),
        )
}
