/// The actual service layer that does the real work.
pub struct Service {}

impl Service {
    /// Create a new instance of the Service that's ready for use.
    pub const fn new() -> Self {
        Self {}
    }

    /// Start the service listening on the given port number.
    ///
    /// # Parameters
    /// - `port` - The port to listen on.
    pub async fn start(self, _port: u16) {}
}
