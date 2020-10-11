use crate::authorization::{
    service::{claims::SecurityContextClaims, AuthorizationService},
    GenerateSecurityContextUseCase, Principal, SecurityContext, SecurityContextId,
    SignedSecurityContext,
};
use async_trait::async_trait;
use chrono::{Timelike, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

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
        principal: Principal,
    ) -> (SecurityContext, SignedSecurityContext) {
        let issued = Utc::now().with_nanosecond(0).unwrap();
        let expires = issued + self.duration;

        let security_context = SecurityContext {
            id: SecurityContextId::default(),
            principal,
            issued,
            expires,
        };

        let claims = SecurityContextClaims::from(&security_context);
        let signed = encode(
            &Header::new(Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(self.signing_key.0.as_ref()),
        )
        .unwrap();

        tracing::debug!(signed = ?signed, security_context = ?security_context, "Generated security context");

        (security_context, SignedSecurityContext(signed))
    }
}

#[cfg(test)]
mod tests {
    use super::super::SigningKey;
    use super::*;
    use assert2::check;
    use chrono::Duration;

    #[test]
    fn test_generate_for_user() {
        let principal = Principal::User("someUserId".to_owned());

        let sut = AuthorizationService::new(Duration::days(3), SigningKey::new("hello"));

        let (security_context, signed) = sut.generate_security_context(principal.clone());

        check!(security_context.principal == principal);
        check!(security_context.expires == security_context.issued + Duration::days(3));
        check!(signed.0 != "");
    }
}
