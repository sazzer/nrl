use super::{Principal, SecurityContext, SignedSecurityContext};
use async_trait::async_trait;

/// The `GenerateSecurityContextUseCase` use case will generate a valid `SecurityContext` for a provided `Principal`
///
/// # Parameters
/// - `principal` - The principal that the security context is for
///
/// # Returns
/// Tuple consisting of the security context and the signed version.
#[async_trait]
pub trait GenerateSecurityContextUseCase {
    fn generate_security_context(
        &self,
        principal: Principal,
    ) -> (SecurityContext, SignedSecurityContext);
}

/// Errors that can occur when authorizing a security context
#[derive(thiserror::Error, Debug)]
pub enum AuthorizeError {
    #[error("An unknown error occurred")]
    UnknownError,
}

/// The `AuthorizeSecurityContextUseCase` use case will assert that a signed security context is valid and return the
/// parsed version if so
///
/// # Parameters
/// - `security_context` The signed security context to authorize
///
/// # Returns
/// The parsed `SecurityContext` if it was valid.
#[async_trait]
pub trait AuthorizeSecurityContextUseCase {
    fn authorize(
        &self,
        security_context: SignedSecurityContext,
    ) -> Result<SecurityContext, AuthorizeError>;
}
