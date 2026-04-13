use axum::debug_handler;

use crate::{
    response::{ApiResult, resp::ApiResponse},
    state::AppState,
};

// Define a route for getting the current version of the API
pub fn get_healthy_router() -> axum::Router<AppState> {
    axum::Router::new().route("/healthy", axum::routing::get(get_healthy_handler))
}

#[debug_handler]
pub async fn get_healthy_handler() -> ApiResult<ApiResponse<String>> {
    Ok(ApiResponse::success("healthy".to_string()))
}
