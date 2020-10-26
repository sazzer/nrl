use super::GoogleConfig;
use crate::authentication::Authenticator;

/// Implementation of the Authenticator for authenticating against Google.
pub struct GoogleAuthenticator {
    pub(super) config: GoogleConfig,
}

impl GoogleAuthenticator {
    /// Create a new instance of the Google Authenticator.
    pub const fn new(config: GoogleConfig) -> Self {
        Self { config }
    }
}

impl Authenticator for GoogleAuthenticator {}
