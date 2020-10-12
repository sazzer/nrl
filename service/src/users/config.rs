use super::{repository::UsersRepository, service::UsersService};
use crate::infrastructure::database::Database;
use crate::infrastructure::server::ServerConfig;
use actix_web::web;
use std::sync::Arc;

/// Configuration for the users component.
pub struct Config {
    _service: Arc<UsersService>,
}

impl Config {
    /// Create a new health component.
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            _service: Arc::new(UsersService::new(UsersRepository::new(database))),
        }
    }
}

impl ServerConfig for Config {
    /// Configure the HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` object to configure
    fn configure(&self, _config: &mut web::ServiceConfig) {}
}
