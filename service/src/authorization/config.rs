use super::{
    service::{AuthorizationService, SigningKey},
    AuthorizeSecurityContextUseCase, GenerateSecurityContextUseCase,
};
use crate::infrastructure::server::ServerConfig;
use actix_web::web;
use chrono::Duration;
use std::sync::Arc;

/// Configuration for the authorization component.
pub struct Config {
    service: Arc<AuthorizationService>,
}

impl Config {
    /// Create a new health component.
    pub fn new() -> Self {
        let duration = Duration::days(365);
        let signing_key = SigningKey::new("Hello");

        Self {
            service: Arc::new(AuthorizationService::new(duration, signing_key)),
        }
    }
}

impl ServerConfig for Config {
    /// Configure the HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` object to configure
    fn configure(&self, config: &mut web::ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn AuthorizeSecurityContextUseCase>);
        config.data(self.service.clone() as Arc<dyn GenerateSecurityContextUseCase>);
    }
}
