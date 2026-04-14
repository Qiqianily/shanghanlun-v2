use crate::{handlers::relations, state::AppState};

pub fn create_treatise_prescriptions_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/query/info/id/{id}",
            axum::routing::get(relations::query::query_treatise_prescriptions_by_id_handler),
        )
        .route(
            "/query/treatises/info/by/name",
            axum::routing::post(relations::query::find_treatises_by_prescription_name_handler),
        )
        .route(
            "/query/treatises/by/prescription/ingredients",
            axum::routing::post(
                relations::query::find_treatises_by_prescription_ingredients_handler,
            ),
        )
        .route(
            "/query/treatise/prescriptions/paginate",
            axum::routing::post(relations::query::query_treatise_prescription_pages_handler),
        )
}
