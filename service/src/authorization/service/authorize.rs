use crate::authorization::{
    service::AuthorizationService, AuthorizeError, AuthorizeSecurityContextUseCase,
    SecurityContext, SignedSecurityContext,
};
use async_trait::async_trait;

#[async_trait]
impl AuthorizeSecurityContextUseCase for AuthorizationService {
    /// Authorize that the provided security context is valid and return the decoded version.
    ///
    /// # Parameters
    /// - `security_context` The signed security context to authorize
    ///
    /// # Returns
    /// The parsed `SecurityContext` if it was valid.
    fn authorize(
        &self,
        _security_context: SignedSecurityContext,
    ) -> Result<SecurityContext, AuthorizeError> {
        todo!()
    }
}
