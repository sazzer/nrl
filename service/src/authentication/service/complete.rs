use crate::authentication::{
    AuthenticationService, AuthenticatorID, CompleteAuthenticationError,
    CompleteAuthenticationUseCase,
};
use crate::users::UserModel;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
impl CompleteAuthenticationUseCase for AuthenticationService {
    /// Complete authentication with the requested authenticator.
    ///
    /// # Parameters
    /// - `authenticator` - The ID of the authenticator to use
    /// - `params` - The parameters received from the authenticator
    ///
    /// # Returns
    /// The details of the newly authenticated user
    /// If an error occurs then details of what the error was.
    async fn complete(
        &self,
        authenticator_id: AuthenticatorID,
        params: HashMap<String, String>,
    ) -> Result<UserModel, CompleteAuthenticationError> {
        let authenticator = self
            .repository
            .get(&authenticator_id)
            .ok_or(CompleteAuthenticationError::UnknownAuthenticator)?;

        let user_details = authenticator
            .complete_authentication(params)
            .await
            .ok_or(CompleteAuthenticationError::AuthenticationFailure)?;

        tracing::debug!(user = ?user_details, authenticator = ?authenticator_id, "Authenticated successfully");

        Err(CompleteAuthenticationError::AuthenticationFailure)
    }
}
