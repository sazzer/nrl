use chrono::{DateTime, Utc};

/// A `Principal` is some entity that has been authorized to perform actions.
pub enum Principal {
    /// A User Principal is a Principal represnting an actual end user.
    User(String),
}

/// A `SecurityContext` represents some authorization credentials issued to some principal.
pub struct SecurityContext {
    /// The Principal that the security context is for
    pub principal: Principal,
    /// When the security context was issued
    pub issued: DateTime<Utc>,
    /// When the security context expires
    pub expires: DateTime<Utc>,
}

/// A `SignedSecurityContext` represents a security context that has been securely signed.
pub struct SignedSecurityContext(String);
