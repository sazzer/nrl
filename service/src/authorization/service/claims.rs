use crate::authorization::{Principal, SecurityContext};
use serde::{Deserialize, Serialize};

/// Representation of the claims within the JWT that represents a signed security context.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityContextClaims {
    /// The ID of the JWT
    pub(super) jti: String,
    /// The Audience of the JWT
    pub(super) aud: String,
    /// The Issuer of the JWT
    pub(super) iss: String,
    /// The Subject of the JWT - literally the User ID
    pub(super) sub: Option<String>,
    /// When the JWT was issued
    pub(super) iat: i64,
    /// The timestamp before which the JWT is not valid
    pub(super) nbf: i64,
    /// When the JWT expires
    pub(super) exp: i64,
}

impl Default for SecurityContextClaims {
    fn default() -> Self {
        Self {
            jti: "".to_owned(),
            aud: "tag:nrl,2020:authorization".to_owned(),
            iss: "tag:nrl,2020:authorization".to_owned(),
            sub: None,
            iat: 0,
            nbf: 0,
            exp: 0,
        }
    }
}

impl From<&SecurityContext> for SecurityContextClaims {
    /// Convert a Security Context into a set of claims that are ready to be signed
    ///
    /// # Parameters
    /// - `security_context` - The security context to convert
    ///
    /// # Returns
    /// The set of claims
    fn from(security_context: &SecurityContext) -> Self {
        Self {
            jti: security_context.id.0.to_string(),
            sub: match &security_context.principal {
                Principal::User(user_id) => Some(user_id.clone()),
            },
            iat: security_context.issued.timestamp(),
            nbf: security_context.issued.timestamp(),
            exp: security_context.expires.timestamp(),
            ..Self::default()
        }
    }
}
