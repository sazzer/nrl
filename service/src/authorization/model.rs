use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use uritemplate::{IntoTemplateVar, TemplateVar};
use uuid::Uuid;

/// A `Principal` is some entity that has been authorized to perform actions.
#[derive(Debug, PartialEq, Clone)]
pub enum Principal {
    /// A User Principal is a Principal represnting an actual end user.
    User(String),
}

/// The ID of a Security Context.
#[derive(Debug, PartialEq)]
pub struct SecurityContextId(pub(super) String);

impl Default for SecurityContextId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

/// A `SecurityContext` represents some authorization credentials issued to some principal.
#[derive(Debug, PartialEq)]
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
#[derive(Debug)]
pub struct SignedSecurityContext(pub(super) String);

impl Display for SignedSecurityContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> IntoTemplateVar for SignedSecurityContext {
    fn into_template_var(self) -> TemplateVar {
        TemplateVar::Scalar(self.0)
    }
}
