use crate::{
    response::{ApiResult, errors::ApiError},
    state::AppState,
};

pub mod healthy;
pub mod prescription;
pub mod show_api;
pub mod treatise;
pub mod version;

/// combine all the routes into one router
pub fn merge_router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/get/current", version::get_version_router())
        .nest("/get/current", healthy::get_healthy_router())
        .nest("/treatise", treatise::create_treatise_router())
        .nest("/prescription", prescription::create_prescription_router())
        .fallback(async || -> ApiResult<()> {
            // 路径找不到
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        }) // 方法不允许
}
