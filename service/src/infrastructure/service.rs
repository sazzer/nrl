use super::server::Server;
use crate::infrastructure::health;
use std::collections::HashMap;
use std::sync::Arc;

/// The actual service layer that does the real work.
pub struct Service {
    server: Server,
}

/// The settings needed for the service to work.
pub struct ServiceSettings {}

impl Service {
    /// Create a new instance of the Service that's ready for use.
    ///
    /// # Parameters
    /// - `settings` - The settingsneeded for the service to work.
    ///
    /// # Returns
    /// The service, ready to work with.
    #[must_use]
    pub fn new(_settings: &ServiceSettings) -> Self {
        let mut health_components: HashMap<String, Arc<dyn health::Healthchecker>> = HashMap::new();
        health_components.insert("db".to_owned(), Arc::new(health::ComponentHealth::Healthy));
        let health = Arc::new(health::Config::new(health_components));

        let server = Server::new().with_config(health);

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
