use crate::authorization::{
    service::AuthorizationService, GenerateSecurityContextUseCase, Principal, SecurityContext,
    SignedSecurityContext,
};
use async_trait::async_trait;

#[async_trait]
impl GenerateSecurityContextUseCase for AuthorizationService {
    /// Generate a new security context for the provided principal.
    ///
    /// # Parameters
    /// - `principal` - The principal that the security context is for
    ///
    /// # Returns
    /// Tuple consisting of the security context and the signed version.
    fn generate_security_context(
        &self,
        _principal: Principal,
    ) -> (SecurityContext, SignedSecurityContext) {
        todo!()
    }
}
