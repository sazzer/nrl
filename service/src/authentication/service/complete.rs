use crate::authentication::{
    AuthenticationService, AuthenticatorID, CompleteAuthenticationError,
    CompleteAuthenticationUseCase,
};
use crate::authorization::{
    GenerateSecurityContextUseCase, SecurityContext, SignedSecurityContext,
};
use crate::users::{AuthenticateUserError, AuthenticateUserUseCase, Authentication, UserModel};
use async_trait::async_trait;
use std::collections::HashMap;
use std::convert::TryInto;

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
    ) -> Result<(UserModel, SecurityContext, SignedSecurityContext), CompleteAuthenticationError>
    {
        let authenticator = self
            .repository
            .get(&authenticator_id)
            .ok_or(CompleteAuthenticationError::UnknownAuthenticator)?;

        let user_details = authenticator
            .complete_authentication(params)
            .await
            .ok_or(CompleteAuthenticationError::AuthenticationFailure)?;

        let user = self
            .users_service
            .authenticate_user(
                Authentication {
                    provider: authenticator_id
                        .try_into()
                        .map_err(|_| CompleteAuthenticationError::UnexpectedError)?,
                    user: user_details.authenticated_user_id,
                    display_name: user_details.authenticated_display_name,
                },
                user_details.user_display_name,
                user_details.user_email,
            )
            .await
            .map_err(|e| {
                tracing::warn!(e = ?e, "An error occurred authenticating the user");
                match e {
                    AuthenticateUserError::DuplicateEmail => {
                        CompleteAuthenticationError::DuplicateEmail
                    }
                    AuthenticateUserError::UnknownError => {
                        CompleteAuthenticationError::UnexpectedError
                    }
                }
            })?;

        let (security_context, signed_security_context) = self
            .authorization_service
            .generate_security_context((&user.identity.id).into());

        tracing::debug!(user = ?user, "Successfully authenticated user");

        Ok((user, security_context, signed_security_context))
    }
}
