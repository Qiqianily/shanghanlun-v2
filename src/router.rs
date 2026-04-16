use crate::{
    response::{ApiResult, errors::ApiError},
    state::AppState,
};

pub mod explanation;
pub mod formula;
pub mod healthy;
pub mod herbs;
pub mod prescription;
pub mod relations;
pub mod show_api;
pub mod sub_formula;
pub mod syndrome;
pub mod treatise;
pub mod version;

/// combine all the routes into one router
pub fn merge_router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/get/current", version::get_version_router())
        .nest("/get/current", healthy::get_healthy_router())
        .nest("/treatise", treatise::create_treatise_router())
        .nest("/prescription", prescription::create_prescription_router())
        .nest(
            "/relations",
            relations::create_treatise_prescriptions_router(),
        )
        .nest("/herbs", herbs::create_treatise_router())
        .nest("/syndrome", syndrome::create_syndrome_router())
        .nest("/formula", formula::create_formula_router())
        .nest("/sub_formula", sub_formula::create_sub_formula_router())
        .nest(
            "/explanations",
            explanation::hu::create_explanation_hu_router(),
        )
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
