use crate::authentication::{
    AuthenticationService, AuthenticatorID, StartAuthentication, StartAuthenticationError,
    StartAuthenticationUseCase,
};

impl StartAuthenticationUseCase for AuthenticationService {
    fn start(
        &self,
        authenticator: AuthenticatorID,
    ) -> Result<StartAuthentication, StartAuthenticationError> {
        self.repository
            .get(&authenticator)
            .ok_or(StartAuthenticationError::UnknownAuthenticator)?;

        todo!()
    }
}
