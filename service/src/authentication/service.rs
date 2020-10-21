mod list;

use crate::authentication::repository::AuthenticatorRepository;

/// The actual authentication service.
pub struct AuthenticationService {
    repository: AuthenticatorRepository,
}

impl AuthenticationService {
    /// Create a new Authentication service.
    pub const fn new(repository: AuthenticatorRepository) -> Self {
        Self { repository }
    }
}
