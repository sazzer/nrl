use crate::authentication::{
    AuthenticationService, AuthenticatorID, StartAuthenticationDetails, StartAuthenticationError,
    StartAuthenticationUseCase,
};
use uuid::Uuid;

impl StartAuthenticationUseCase for AuthenticationService {
    fn start(
        &self,
        authenticator: AuthenticatorID,
    ) -> Result<StartAuthenticationDetails, StartAuthenticationError> {
        let authenticator = self
            .repository
            .get(&authenticator)
            .ok_or(StartAuthenticationError::UnknownAuthenticator)?;

        let state = Uuid::new_v4().to_string();

        let redirect_uri = authenticator.start_authentication(&state);

        Ok(StartAuthenticationDetails {
            state,
            redirect_uri,
        })
    }
}
