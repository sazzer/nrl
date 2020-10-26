use crate::authentication::{repository::AuthenticatorRepository, service::AuthenticationService};
use crate::infrastructure::server::ServerConfig;
use actix_web::web;
use std::sync::Arc;

/// Configuration for the authentication component.
pub struct BuiltConfig {
    service: Arc<AuthenticationService>,
}

pub struct Config {
    service: AuthenticationService,
}

impl Config {
    /// Create a new authentication component.
    pub fn new() -> Self {
        Self {
            service: AuthenticationService::new(AuthenticatorRepository::new()),
        }
    }

    /// Convert this object into the built config for use by other components.
    pub fn build(self) -> BuiltConfig {
        BuiltConfig {
            service: Arc::new(self.service),
        }
    }

    /// Add Google as an authenticator if possible.
    pub fn with_google(mut self, config: Option<super::google::GoogleConfig>) -> Self {
        if let Some(config) = config {
            let authenticator = super::google::GoogleAuthenticator::new(config);

            self.service
                .with_authenticator("google".parse().unwrap(), Arc::new(authenticator));
        }

        self
    }
}

impl ServerConfig for BuiltConfig {
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
        config.service(
            web::resource("/authentication/{id}")
                .route(web::get().to(super::endpoints::start_authentication)),
        );
        config.service(
            web::resource("/authentication/{id}/complete")
                .route(web::get().to(super::endpoints::complete_authentication)),
        );
    }
}
