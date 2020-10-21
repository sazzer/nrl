use super::AuthenticationService;
use crate::authentication::{AuthenticatorID, ListProvidersUseCase};

impl ListProvidersUseCase for AuthenticationService {
    /// Get the list of all authenticators that are known.
    fn list(&self) -> Vec<&AuthenticatorID> {
        self.registry.list()
    }
}
