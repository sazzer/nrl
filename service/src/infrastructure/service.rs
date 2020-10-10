use super::{database::Database, server::Server};
use crate::infrastructure::health;
use std::collections::HashMap;
use std::sync::Arc;

/// The actual service layer that does the real work.
pub struct Service {
    pub(super) server: Server,
}

/// The settings needed for the service to work.
#[derive(Debug)]
pub struct ServiceSettings {
    /// The database connection URL.
    pub database_url: String,
}

impl Service {
    /// Create a new instance of the Service that's ready for use.
    ///
    /// # Parameters
    /// - `settings` - The settingsneeded for the service to work.
    ///
    /// # Returns
    /// The service, ready to work with.
    #[must_use]
    pub fn new(settings: &ServiceSettings) -> Self {
        tracing::debug!(settings = ?settings, "Building service");

        let database = Arc::new(Database::new(&settings.database_url));

        let mut health_components: HashMap<String, Arc<dyn health::Healthchecker>> = HashMap::new();
        health_components.insert("db".to_owned(), database);
        let health = Arc::new(health::Config::new(health_components));

        let server = Server::new().with_config(health);

        tracing::debug!("Finished building service");

        Self { server }
    }

    /// Start the service listening on the given port number.
    ///
    /// # Parameters
    /// - `port` - The port to listen on.
    pub async fn start(self, port: u16) {
        self.server.start(port).await
    }
}
