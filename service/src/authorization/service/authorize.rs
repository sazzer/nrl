use crate::authorization::{
    service::{claims::SecurityContextClaims, AuthorizationService},
    AuthorizeError, AuthorizeSecurityContextUseCase, SecurityContext, SignedSecurityContext,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::collections::HashSet;
use std::convert::TryInto;

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
        security_context: SignedSecurityContext,
    ) -> Result<SecurityContext, AuthorizeError> {
        let mut valid_audiences = HashSet::new();
        valid_audiences.insert("tag:nrl,2020:authorization".to_owned());

        let token = decode::<SecurityContextClaims>(
            &security_context.0,
            &DecodingKey::from_secret(self.signing_key.0.as_ref()),
            &Validation {
                iss: Some("tag:nrl,2020:authorization".to_owned()),
                aud: Some(valid_audiences),
                algorithms: vec![Algorithm::HS512],
                ..Validation::default()
            },
        )
        .map_err(|err| {
            tracing::warn!(err = ?err, security_context = ?security_context, "Error verifying security context");

            match err.into_kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthorizeError::Expired,
                jsonwebtoken::errors::ErrorKind::InvalidSignature |
                jsonwebtoken::errors::ErrorKind::InvalidToken |
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithm |
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName |
                jsonwebtoken::errors::ErrorKind::InvalidAudience |
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => AuthorizeError::Malformed,
                _ => AuthorizeError::UnknownError,
            }
        })?;

        token.claims.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authorization::{
        service::SigningKey, AuthorizeError, AuthorizeSecurityContextUseCase,
        GenerateSecurityContextUseCase, Principal, SecurityContext, SecurityContextId,
        SignedSecurityContext,
    };
    use assert2::check;
    use chrono::Duration;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    const EARLY_TIMESTAMP: i64 = 1_496_437_337;
    const LATE_TIMESTAMP: i64 = 2_496_437_337;

    #[test]
    fn test_generate_authorize() {
        let principal = Principal::User("someUserId".to_owned());

        let sut = AuthorizationService::new(Duration::days(3), SigningKey::new("hello"));

        let (security_context, signed) = sut.generate_security_context(principal);

        let result = sut.authorize(signed);
        check!(result.unwrap() == security_context);
    }

    #[test]
    fn test_authorize_success() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap();

        check!(verified.id == SecurityContextId("securityContextId".to_owned()));
        check!(verified.principal == Principal::User("securityContextPrincipal".to_owned()));
        check!(verified.issued.timestamp() == EARLY_TIMESTAMP);
        check!(verified.expires.timestamp() == LATE_TIMESTAMP);
    }

    #[test]
    fn test_verify_no_principal() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: None,
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == AuthorizeError::MissingPrincipal);
    }

    #[test]
    fn test_verify_expired() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: EARLY_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == AuthorizeError::Expired);
    }

    #[test]
    fn test_verify_wrong_key() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test2",
        )
        .unwrap_err();

        check!(verified == AuthorizeError::Malformed);
    }

    #[test]
    fn test_verify_wrong_algorithm() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS256),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == AuthorizeError::Malformed);
    }

    #[test]
    fn test_verify_wrong_audience() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    aud: "incorrect".to_owned(),
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == AuthorizeError::Malformed);
    }

    #[test]
    fn test_verify_wrong_issuer() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    iss: "incorrect".to_owned(),
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == AuthorizeError::Malformed);
    }

    #[test]
    fn test_verify_malformed_token() {
        let verified = help_verify_token("malformed".to_owned(), "test2").unwrap_err();

        check!(verified == AuthorizeError::Malformed);
    }

    fn help_verify_token(token: String, secret: &str) -> Result<SecurityContext, AuthorizeError> {
        let sut = AuthorizationService::new(Duration::days(3), SigningKey::new(secret));

        let signed = SignedSecurityContext(token);

        sut.authorize(signed)
    }
}
