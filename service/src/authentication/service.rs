mod complete;
mod list;
mod start;

use crate::authentication::{repository::AuthenticatorRepository, Authenticator, AuthenticatorID};
use crate::users::UsersService;
use std::sync::Arc;

/// The actual authentication service.
pub struct AuthenticationService {
    repository: AuthenticatorRepository,
    users_service: Arc<UsersService>,
}

impl AuthenticationService {
    /// Create a new Authentication service.
    pub const fn new(
        repository: AuthenticatorRepository,
        users_service: Arc<UsersService>,
    ) -> Self {
        Self {
            repository,
            users_service,
        }
    }

    /// Add a new authenticator to the repository.
    ///
    /// # Parameters
    /// - `authenticator_id` - The ID of the authenticator
    /// - `authenticator` - The authenticator itself
    pub fn with_authenticator(
        &mut self,
        authenticator_id: AuthenticatorID,
        authenticator: Arc<dyn Authenticator>,
    ) -> &mut Self {
        self.repository
            .with_authenticator(authenticator_id, authenticator);
        self
    }
}
