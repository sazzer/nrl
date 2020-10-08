use crate::infrastructure::server::ServerConfig;
use actix_web::web;

/// Configuration for the healthchecks component.
#[derive(Default)]
pub struct Config {}

impl ServerConfig for Config {
    /// Configure the HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` object to configure
    fn configure(&self, config: &mut web::ServiceConfig) {
        config.service(web::resource("/health").route(web::get().to(super::endpoints::get_health)));
    }
}
