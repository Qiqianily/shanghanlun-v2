use crate::{handlers::treatise, state::AppState};

/// 创建原文相关的路由，专门用来管理与原文相关的操作
pub fn create_treatise_router() -> axum::Router<AppState> {
    axum::Router::new()
        // .route(
        //     "/update/content/id/{id}",
        //     axum::routing::put(treatise::update::update_treatise_content_handler),
        // )
        // .route_layer(get_auth_layer()) // 这里加上权限认证
        .route(
            "/query/info/id/{id}",
            axum::routing::get(treatise::query::query_treatise_by_id_handler),
        )
        .route(
            "/query/pages/infos",
            axum::routing::post(treatise::query::query_treatise_pagination_handler),
        )
        .route(
            "/query/like/infos",
            axum::routing::get(treatise::query::query_treatise_by_keyword_handler),
        )
}
