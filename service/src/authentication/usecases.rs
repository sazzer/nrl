use crate::authentication::AuthenticatorID;
use crate::authorization::{SecurityContext, SignedSecurityContext};
use crate::users::UserModel;
use async_trait::async_trait;
use std::collections::HashMap;

/// Use Case for listing the available authentication providers.
pub trait ListProvidersUseCase {
    /// Get the list of all authenticators that are known.
    fn list(&self) -> Vec<&AuthenticatorID>;
}

/// The result of starting authentication.
#[derive(Debug)]
pub struct StartAuthenticationDetails {
    pub redirect_uri: String,
    pub state: String,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum StartAuthenticationError {
    #[error("The requested authenticator was unknown")]
    UnknownAuthenticator,
}

pub trait StartAuthenticationUseCase {
    /// Start authentication with the requested authenticator.
    ///
    /// # Parameters
    /// - `authenticator` - The ID of the authenticator to use
    ///
    /// # Returns
    /// The details required to redirect the client to start authentication.
    /// If an error occurs then details of what the error was.
    fn start(
        &self,
        authenticator: AuthenticatorID,
    ) -> Result<StartAuthenticationDetails, StartAuthenticationError>;
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CompleteAuthenticationError {
    #[error("The requested authenticator was unknown")]
    UnknownAuthenticator,

    #[error("The user failed to authenticate")]
    AuthenticationFailure,

    #[error("Registering user with a duplicate email address")]
    DuplicateEmail,

    #[error("An unexpected error occurred")]
    UnexpectedError,
}

#[async_trait]
pub trait CompleteAuthenticationUseCase {
    /// Complete authentication with the requested authenticator.
    ///
    /// # Parameters
    /// - `authenticator` - The ID of the authenticator to use
    /// - `params` - The parameters received from the authenticator
    ///
    /// # Returns
    /// The details of the newly authenticated user
    /// If an error occurs then details of what the error was.
    async fn complete(
        &self,
        authenticator: AuthenticatorID,
        params: HashMap<String, String>,
    ) -> Result<(UserModel, SecurityContext, SignedSecurityContext), CompleteAuthenticationError>;
}
