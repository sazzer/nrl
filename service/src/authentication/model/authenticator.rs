use crate::users::{DisplayName, Email, ProviderUserID};
use async_trait::async_trait;
use std::collections::HashMap;

/// Details of a user that has just authenticated.
#[derive(Debug)]
pub struct AuthenticatedUser {
    pub authenticated_user_id: ProviderUserID,
    pub authenticated_display_name: String,
    pub user_display_name: DisplayName,
    pub user_email: Option<Email>,
}

/// Trait for starting authentication.
pub trait StartAuthentication: Send + Sync {
    /// Start authentication with the provider.
    ///
    /// # Parameters
    /// - `state` - A state value that is unique to this request
    ///
    /// # Returns
    /// The URI to redirect the client to in order to start authentication
    fn start_authentication(&self, state: &str) -> String;
}

/// Trait for completing authentication.
#[async_trait]
pub trait CompleteAuthentication: Send + Sync {
    /// Complete authentication with the provider
    ///
    /// # Parameters
    /// - `params` - The parameters received from the provider
    ///
    /// # Returns
    /// The details of the user that just authenticated.
    /// If authentication failed then returns `None` instead.
    async fn complete_authentication(
        &self,
        params: HashMap<String, String>,
    ) -> Option<AuthenticatedUser>;
}

/// Trait that all authenticators must implement.
pub trait Authenticator: StartAuthentication + CompleteAuthentication {}
