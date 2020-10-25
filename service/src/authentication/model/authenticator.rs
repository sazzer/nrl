use crate::users::{DisplayName, Email, ProviderUserID};
use async_trait::async_trait;
use std::collections::HashMap;

/// Details of a user that has just authenticated.
pub struct AuthenticatedUser {
    pub user_id: ProviderUserID,
    pub display_name: DisplayName,
    pub email: Option<Email>,
}

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

    /// Complete authentication with the provider
    ///
    /// # Parameters
    /// - `params` - The parameters received from the provider
    ///
    /// # Returns
    /// The details of the user that just authenticated.
    async fn complete_authentication(&self, params: HashMap<String, String>) -> AuthenticatedUser;
}
