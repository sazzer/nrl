use super::super::CheckHealthUseCase;
use super::model::SystemHealthModel;
use actix_web::web::Data;
use std::sync::Arc;

/// HTTP Handler for checking the health of the system.
///
/// # Parameters
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/health"),
    skip(health_service)
)]
pub async fn get_health(health_service: Data<Arc<dyn CheckHealthUseCase>>) -> SystemHealthModel {
    let health = health_service.check_health().await;

    tracing::debug!(health = ?health, "System health");

    health.into()
}
