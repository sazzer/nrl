mod authorize;
mod claims;
mod generate;
mod signing_key;

use chrono::Duration;
pub(super) use signing_key::SigningKey;

/// The Authorization Service.
pub struct AuthorizationService {
    duration: Duration,
    signing_key: SigningKey,
}

impl AuthorizationService {
    /// Create a new authorization service.
    pub const fn new(duration: Duration, signing_key: SigningKey) -> Self {
        Self {
            duration,
            signing_key,
        }
    }
}
