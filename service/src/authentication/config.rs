use super::Registry;
use crate::infrastructure::server::ServerConfig;
use actix_web::web;
use std::sync::Arc;

/// Configuration for the authentication component.
pub struct Config {
    registry: Arc<Registry>,
}

impl Config {
    /// Create a new authentication component.
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Registry::new()),
        }
    }
}

impl ServerConfig for Config {
    /// Configure the HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` object to configure
    fn configure(&self, config: &mut web::ServiceConfig) {
        config.data(self.registry.clone());
    }
}
