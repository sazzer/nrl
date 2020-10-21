use crate::authentication::{repository::AuthenticatorRepository, service::AuthenticationService};
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
            service: Arc::new(AuthenticationService::new(AuthenticatorRepository::new())),
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

        config.service(
            web::resource("/authentication")
                .route(web::get().to(super::endpoints::list_authenticators)),
        );
    }
}
