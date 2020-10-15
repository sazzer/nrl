use super::{repository::UsersRepository, service::UsersService};
use crate::infrastructure::database::Database;
use crate::infrastructure::server::ServerConfig;
use actix_web::web;
use std::sync::Arc;

/// Configuration for the users component.
pub struct Config {
    service: Arc<UsersService>,
}

impl Config {
    /// Create a new health component.
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            service: Arc::new(UsersService::new(UsersRepository::new(database))),
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
            web::resource("/users/{id}").route(web::get().to(super::endpoints::get_user_by_id)),
        );
    }
}
