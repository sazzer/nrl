use crate::authentication::AuthenticatorID;

/// Use Case for listing the available authentication providers.
pub trait ListProvidersUseCase {
    /// Get the list of all authenticators that are known.
    fn list(&self) -> Vec<&AuthenticatorID>;
}
