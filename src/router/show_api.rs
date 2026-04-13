pub async fn show_api() -> anyhow::Result<()> {
    tracing::info!("GET /api/v1/get/current/version");
    tracing::info!("GET /api/v1/get/current/healthy");
    Ok(())
}
