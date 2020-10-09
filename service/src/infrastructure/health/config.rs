use super::{service::HealthService, CheckHealthUseCase, Healthchecker};
use crate::infrastructure::server::ServerConfig;
use actix_web::web;
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for the healthchecks component.
pub struct Config {
    service: Arc<HealthService>,
}

impl Config {
    /// Create a new health component.
    pub fn new(components: HashMap<String, Arc<dyn Healthchecker>>) -> Self {
        Self {
            service: Arc::new(HealthService::new(components)),
        }
    }
}

impl ServerConfig for Config {
    /// Configure the HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` object to configure
    fn configure(&self, config: &mut web::ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn CheckHealthUseCase>);

        config.service(web::resource("/health").route(web::get().to(super::endpoints::get_health)));
    }
}
