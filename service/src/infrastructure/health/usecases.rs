use super::SystemHealth;
use async_trait::async_trait;

/// Use Case for checking the health of the system
#[async_trait]
pub trait CheckHealthUseCase {
    /// Check the health of the system
    ///
    /// # Returns
    /// The overall system health.
    async fn check_health(&self) -> SystemHealth;
}
