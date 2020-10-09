use super::{CheckHealthUseCase, ComponentHealth, Healthchecker, SystemHealth};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// The actual health service.
pub struct HealthService {
    /// The components that can have their health checked.
    components: HashMap<String, Arc<dyn Healthchecker>>,
}

impl HealthService {
    /// Create a new health service.
    ///
    /// # Parameters
    /// - `components` - The components that can have their health checked.
    pub fn new(components: HashMap<String, Arc<dyn Healthchecker>>) -> Self {
        Self { components }
    }
}
#[async_trait]
impl CheckHealthUseCase for HealthService {
    /// Check the health of the system
    ///
    /// # Returns
    /// The overall system health.
    async fn check_health(&self) -> SystemHealth {
        let mut components = HashMap::new();

        for (name, component) in &self.components {
            let health = match component.check_health().await {
                Ok(()) => ComponentHealth::Healthy,
                Err(err) => ComponentHealth::Unhealthy(err),
            };

            tracing::debug!(name = ?name, health = ?health, "Component health");

            components.insert(name.clone(), health);
        }

        SystemHealth::new(components)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[actix_rt::test]
    pub async fn test_empty_system() {
        let components = HashMap::new();
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(result.healthy());
        check!(result.components.len() == 0);
    }

    #[actix_rt::test]
    pub async fn test_healthy_system() {
        let mut components: HashMap<String, Arc<dyn Healthchecker>> = HashMap::new();
        components.insert("healthy".to_owned(), Arc::new(ComponentHealth::Healthy));
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(result.healthy());
        check!(result.components.len() == 1);
        check!(result.components["healthy"] == ComponentHealth::Healthy);
    }

    #[actix_rt::test]
    pub async fn test_unhealthy_system() {
        let mut components: HashMap<String, Arc<dyn Healthchecker>> = HashMap::new();
        components.insert(
            "unhealthy".to_owned(),
            Arc::new(ComponentHealth::Unhealthy("Oops".to_owned())),
        );
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(!result.healthy());
        check!(result.components.len() == 1);
        check!(result.components["unhealthy"] == ComponentHealth::Unhealthy("Oops".to_owned()));
    }

    #[actix_rt::test]
    pub async fn test_mixed_system() {
        let mut components: HashMap<String, Arc<dyn Healthchecker>> = HashMap::new();
        components.insert("healthy".to_owned(), Arc::new(ComponentHealth::Healthy));
        components.insert(
            "unhealthy".to_owned(),
            Arc::new(ComponentHealth::Unhealthy("Oops".to_owned())),
        );
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(!result.healthy());
        check!(result.components.len() == 2);
        check!(result.components["healthy"] == ComponentHealth::Healthy);
        check!(result.components["unhealthy"] == ComponentHealth::Unhealthy("Oops".to_owned()));
    }
}
