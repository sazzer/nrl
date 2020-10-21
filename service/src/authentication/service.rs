mod list;
pub mod registry;

use registry::Registry;

/// The actual authentication service.
pub struct AuthenticationService {
    registry: Registry,
}

impl AuthenticationService {
    /// Create a new Authentication service.
    pub const fn new(registry: Registry) -> Self {
        Self { registry }
    }
}
