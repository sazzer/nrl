use async_trait::async_trait;

/// Trait that all authenticators can implement.
#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Start authentication with the provider.
    ///
    /// # Parameters
    /// - `state` - A state value that is unique to this request
    ///
    /// # Returns
    /// The URI to redirect the client to in order to start authentication
    fn start_authentication(&self, state: &str) -> String;
}
