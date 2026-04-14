pub async fn show_api() -> anyhow::Result<()> {
    tracing::info!("GET /api/v1/get/current/version");
    tracing::info!("GET /api/v1/get/current/healthy");
    tracing::info!("GET /api/v1/treatise/query/info/id/:id");
    tracing::info!("GET /api/v1/treatise/query/pages/infos");
    tracing::info!("GET /api/v1/treatise/query/like/infos");
    Ok(())
}
