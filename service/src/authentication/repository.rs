use crate::authentication::{Authenticator, AuthenticatorID};
use std::collections::HashMap;
use std::sync::Arc;

/// The repository of authenticators that we can use
pub struct AuthenticatorRepository {
    authenticators: HashMap<AuthenticatorID, Arc<dyn Authenticator>>,
}

impl AuthenticatorRepository {
    /// Create a new repository of authenticators
    pub fn new() -> Self {
        Self {
            authenticators: HashMap::new(),
        }
    }

    /// Add a new authenticator to the repository.
    ///
    /// # Parameters
    /// - `authenticator_id` - The ID of the authenticator
    /// - `authenticator` - The authenticator itself
    pub fn with_authenticator(
        &mut self,
        authenticator_id: AuthenticatorID,
        authenticator: Arc<dyn Authenticator>,
    ) -> &mut Self {
        self.authenticators.insert(authenticator_id, authenticator);
        self
    }

    /// Get the list of all authenticators that are present in the repository.
    pub fn list(&self) -> Vec<&AuthenticatorID> {
        self.authenticators.keys().collect()
    }

    /// Get the authenticator that has the provided ID
    ///
    /// # Parameters
    /// - `index` - The ID of the authenticator to get
    ///
    /// # Returns
    /// The authenticator, or `None` if it is unknown.
    pub fn get(&self, index: &AuthenticatorID) -> Option<&Arc<dyn Authenticator>> {
        self.authenticators.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::AuthenticatedUser;
    use assert2::check;
    use async_trait::async_trait;

    #[test]
    fn list_no_authenticators() {
        let sut = AuthenticatorRepository::new();

        let result: Vec<&AuthenticatorID> = sut.list();
        check!(result.is_empty());
    }

    #[test]
    fn list_authenticators() {
        let mut sut = AuthenticatorRepository::new();
        sut.with_authenticator("google".parse().unwrap(), Arc::new(MockAuthenticator {}));
        sut.with_authenticator("twitter".parse().unwrap(), Arc::new(MockAuthenticator {}));

        // Note that we're asserting on the output of formattingthe Authenticator IDs, purely because it's easier
        // than sorting them.
        let mut result: Vec<String> = sut.list().into_iter().map(|v| format!("{:?}", v)).collect();
        result.sort();

        check!(result.len() == 2);
        check!(result[0] == "AuthenticatorID(\"google\")");
        check!(result[1] == "AuthenticatorID(\"twitter\")");
    }

    #[test]
    fn get_unknown_authenticator() {
        let sut = AuthenticatorRepository::new();

        let result = sut.get(&"unknown".parse().unwrap());
        check!(result.is_none());
    }

    #[test]
    #[allow(clippy::vtable_address_comparisons)]
    fn get_known_authenticator() {
        let authenticator = Arc::new(MockAuthenticator {});

        let mut sut = AuthenticatorRepository::new();
        sut.with_authenticator("google".parse().unwrap(), authenticator.clone());

        let result = sut.get(&"google".parse().unwrap()).unwrap();

        check!(std::ptr::eq(result.as_ref(), authenticator.as_ref()));
    }

    struct MockAuthenticator {}

    #[async_trait]
    impl Authenticator for MockAuthenticator {
        fn start_authentication(&self, _state: &str) -> String {
            unimplemented!()
        }

        async fn complete_authentication(
            &self,
            _params: HashMap<String, String>,
        ) -> AuthenticatedUser {
            unimplemented!()
        }
    }
}
