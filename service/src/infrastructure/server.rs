/// Wrapper around the HTTP Server.
pub struct Server {}

impl Server {
    /// Construct a new Server.
    pub const fn new() -> Self {
        Self {}
    }

    /// Start the HTTP Server listening on the given port number.
    ///
    /// # Parameters
    /// - `port` - The port to listen on.
    pub async fn start(self, port: u16) {
        tracing::info!(port = port, "Starting NRL");
    }
}
