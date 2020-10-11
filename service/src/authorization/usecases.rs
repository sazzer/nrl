use super::{Principal, SecurityContext, SignedSecurityContext};
use async_trait::async_trait;

/// The `GenerateSecurityContextUseCase` use case will generate a valid `SecurityContext` for a provided `Principal`
#[async_trait]
pub trait GenerateSecurityContextUseCase {
    /// Generate a new security context for the provided principal.
    ///
    /// # Parameters
    /// - `principal` - The principal that the security context is for
    ///
    /// # Returns
    /// Tuple consisting of the security context and the signed version.
    fn generate_security_context(
        &self,
        principal: Principal,
    ) -> (SecurityContext, SignedSecurityContext);
}

/// Errors that can occur when authorizing a security context
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AuthorizeError {
    #[error("The token has expired")]
    Expired,
    #[error("The token was malformed")]
    Malformed,
    #[error("The token was missing a principal")]
    MissingPrincipal,
    #[error("An unknown error occurred")]
    UnknownError,
}

/// The `AuthorizeSecurityContextUseCase` use case will assert that a signed security context is valid and return the
/// parsed version if so
#[async_trait]
pub trait AuthorizeSecurityContextUseCase {
    /// Authorize that the provided security context is valid and return the decoded version.
    ///
    /// # Parameters
    /// - `security_context` The signed security context to authorize
    ///
    /// # Returns
    /// The parsed `SecurityContext` if it was valid.
    fn authorize(
        &self,
        security_context: SignedSecurityContext,
    ) -> Result<SecurityContext, AuthorizeError>;
}
