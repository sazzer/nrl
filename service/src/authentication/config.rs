use super::service::{registry::Registry, AuthenticationService};
use crate::infrastructure::server::ServerConfig;
use actix_web::web;
use std::sync::Arc;

/// Configuration for the authentication component.
pub struct Config {
    service: Arc<AuthenticationService>,
}

impl Config {
    /// Create a new authentication component.
    pub fn new() -> Self {
        Self {
            service: Arc::new(AuthenticationService::new(Registry::new())),
        }
    }
}

impl ServerConfig for Config {
    /// Configure the HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` object to configure
    fn configure(&self, config: &mut web::ServiceConfig) {
        config.data(self.service.clone());
    }
}
