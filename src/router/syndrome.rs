use crate::{handlers::syndrome, state::AppState};

/// 创建原文相关的路由，专门用来管理与原文相关的操作
pub fn create_syndrome_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/query/info/id/{id}",
            axum::routing::get(syndrome::query::query_syndrome_by_id_handler),
        )
        .route(
            "/query/infos/by/title",
            axum::routing::get(syndrome::query::query_syndrome_by_title_or_alias_handler),
        )
        .route(
            "/query/infos/by/prescription",
            axum::routing::get(syndrome::query::query_syndrome_by_prescription_handler),
        )
        .route(
            "/query/infos/by/manifestation",
            axum::routing::get(syndrome::query::query_syndrome_by_manifestation_handler),
        )
}
