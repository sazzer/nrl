use super::GoogleConfig;
use crate::authentication::Authenticator;

/// Implementation of the Authenticator for authenticating against Google.
pub struct GoogleAuthenticator {
    _config: GoogleConfig,
}

impl GoogleAuthenticator {
    /// Create a new instance of the Google Authenticator.
    pub const fn new(config: GoogleConfig) -> Self {
        Self { _config: config }
    }
}

impl Authenticator for GoogleAuthenticator {}
