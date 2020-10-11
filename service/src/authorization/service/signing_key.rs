/// The key used to sign a JWT.
pub struct SigningKey(pub(super) String);

impl SigningKey {
    /// Create a new signing key.
    ///
    /// # Parameters
    /// - `key` - The raw key
    pub fn new<S>(key: S) -> Self
    where
        S: Into<String>,
    {
        Self(key.into())
    }
}
