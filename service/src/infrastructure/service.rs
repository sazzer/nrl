use super::server::Server;
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
        let health = Arc::new(crate::infrastructure::health::Config::default());

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
