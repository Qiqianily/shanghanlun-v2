pub async fn show_api() -> anyhow::Result<()> {
    tracing::info!("GET   /api/v1/get/current/version");
    tracing::info!("GET   /api/v1/get/current/healthy");
    tracing::info!("GET   /api/v1/treatise/query/info/id/:id");
    tracing::info!("GET   /api/v1/treatise/query/pages/infos");
    tracing::info!("GET   /api/v1/treatise/query/like/infos");
    tracing::info!("GET   /api/v1/prescription/query/info/id/:id");
    tracing::info!("POST  /api/v1/prescription/query/pages/infos/by/function");
    tracing::info!("GET   /api/v1/prescription/query/infos/by/name");
    tracing::info!("POST  /api/v1/prescription/query/infos/by/ingredients");
    tracing::info!("GET   /api/v1/relations/query/info/id/:id");
    tracing::info!("POST  /api/v1/relations/query/treatises/info/by/name");
    tracing::info!("POST  /api/v1/relations/query/treatises/by/prescription/ingredients");
    tracing::info!("POST  /api/v1/relations/query/treatise/prescriptions/paginate");
    tracing::info!("GET   /api/v1/herbs/query/info/id/:id");
    tracing::info!("GET   /api/v1/herbs/query/infos/by/name");
    tracing::info!("GET   /api/v1/herbs/query/infos/from/content");
    tracing::info!("GET   /api/v1/herbs/query/infos/by/category");
    Ok(())
}
