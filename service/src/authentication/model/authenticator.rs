use async_trait::async_trait;

/// Trait that all authenticators can implement.
#[async_trait]
pub trait Authenticator: Send + Sync {}
