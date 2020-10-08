use super::server::Server;

/// The actual service layer that does the real work.
pub struct Service {
    server: Server,
}

impl Service {
    /// Create a new instance of the Service that's ready for use.
    #[must_use]
    pub fn new() -> Self {
        let server = Server::new(vec![]);

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
