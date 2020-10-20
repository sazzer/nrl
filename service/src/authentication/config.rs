use crate::infrastructure::server::ServerConfig;
use actix_web::web;

/// Configuration for the authentication component.
pub struct Config {}

impl Config {
    /// Create a new authentication component.
    pub const fn new() -> Self {
        Self {}
    }
}

impl ServerConfig for Config {
    /// Configure the HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` object to configure
    fn configure(&self, _config: &mut web::ServiceConfig) {}
}
