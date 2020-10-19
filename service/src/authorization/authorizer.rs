use super::{Principal, SecurityContext};
use crate::http::problem::{Problem, SimpleProblemType};
use actix_http::http::StatusCode;

/// The Authorizer is a means to authorize that a request is allowed to perform some actions.
pub struct Authorizer {
    /// The security context for this request, if there is one.
    security_context: Option<SecurityContext>,
}

/// Intermediate structure for authorizing a request.
pub struct Authorizing<'a>(&'a Option<SecurityContext>, bool);

impl Authorizer {
    /// Create a new authorizer.
    ///
    /// # Parameters
    /// - `security_context` - The security context to authorize, if there is onr.
    pub const fn new(security_context: Option<SecurityContext>) -> Self {
        Self { security_context }
    }

    /// Begin authorizing a request.
    ///
    /// # Returns
    /// The `Authorizing` struct, which can then perform actual checks.
    pub const fn begin(&self) -> Authorizing {
        Authorizing(&self.security_context, true)
    }
}

impl<'a> Authorizing<'a> {
    /// Finish authorizing, producing a `Result` to indicate success or failure.
    pub fn into_result(self) -> Result<(), Problem> {
        if self.1 {
            Ok(())
        } else {
            Err(FORBIDDEN.into())
        }
    }

    /// Helper to perform a check and return the appropriate `Authorizing` struct to continue with.
    ///
    /// # Parameters
    /// - `result` - The result of the check
    ///
    /// # Returns
    /// A new `Authorizing` struct to continue authorization check with.
    const fn perform_check(self, result: bool) -> Self {
        Self(self.0, self.1 && result)
    }

    /// Check if this request has been authorized, but regardless of who it was authorized for.
    pub fn authorized(self) -> Self {
        let result = self.0.is_some();
        self.perform_check(result)
    }

    /// Check if this request has been authorized, but regardless of who it was authorized for.
    pub fn same_principal(self, principal: &Principal) -> Self {
        let result = match &self.0 {
            None => false,
            Some(sc) => &sc.principal == principal,
        };

        self.perform_check(result)
    }
}

pub const FORBIDDEN: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:nrl/2020:problems/forbidden",
    problem_title: "Access to this resource is forbidden",
    status_code: StatusCode::FORBIDDEN,
};
