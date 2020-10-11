use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A `Principal` is some entity that has been authorized to perform actions.
#[derive(Debug, PartialEq, Clone)]
pub enum Principal {
    /// A User Principal is a Principal represnting an actual end user.
    User(String),
}

/// The ID of a Security Context.
#[derive(Debug)]
pub struct SecurityContextId(pub(super) Uuid);

impl Default for SecurityContextId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

/// A `SecurityContext` represents some authorization credentials issued to some principal.
#[derive(Debug)]
pub struct SecurityContext {
    /// The ID of the security context
    pub id: SecurityContextId,
    /// The Principal that the security context is for
    pub principal: Principal,
    /// When the security context was issued
    pub issued: DateTime<Utc>,
    /// When the security context expires
    pub expires: DateTime<Utc>,
}

/// A `SignedSecurityContext` represents a security context that has been securely signed.
pub struct SignedSecurityContext(pub(super) String);
