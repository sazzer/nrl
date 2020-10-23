use crate::authentication::AuthenticatorID;

/// Use Case for listing the available authentication providers.
pub trait ListProvidersUseCase {
    /// Get the list of all authenticators that are known.
    fn list(&self) -> Vec<&AuthenticatorID>;
}

/// The result of starting authentication.
#[derive(Debug)]
pub struct StartAuthentication {
    redirect_uri: String,
    state: String,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum StartAuthenticationError {
    #[error("The requested authenticator was unknown")]
    UnknownAuthenticator,

    #[error("An unexpected error occurred")]
    UnexpectedError,
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
    ) -> Result<StartAuthentication, StartAuthenticationError>;
}
