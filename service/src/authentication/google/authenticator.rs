use super::GoogleConfig;
use crate::authentication::Authenticator;
use uritemplate::UriTemplate;

/// Default URI template for starting authentication if one isn't provided.
const DEFAULT_AUTH_URI: &str = "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}";

/// Implementation of the Authenticator for authenticating against Google.
pub struct GoogleAuthenticator {
    config: GoogleConfig,
}

impl GoogleAuthenticator {
    /// Create a new instance of the Google Authenticator.
    pub const fn new(config: GoogleConfig) -> Self {
        Self { config }
    }
}

impl Authenticator for GoogleAuthenticator {
    /// Start authentication with the provider.
    ///
    /// # Parameters
    /// - `state` - A state value that is unique to this request
    ///
    /// # Returns
    /// The URI to redirect the client to in order to start authentication
    fn start_authentication(&self, state: &str) -> String {
        let auth_uri = self
            .config
            .auth_uri
            .clone()
            .unwrap_or_else(|| DEFAULT_AUTH_URI.to_owned());

        UriTemplate::new(&auth_uri)
            .set("client_id", self.config.client_id.clone())
            .set("response_type", "code")
            .set("scope", "openid email profile")
            .set("redirect_uri", self.config.redirect_uri.clone())
            .set("state", state)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_start_auth() {
        let sut = GoogleAuthenticator::new(GoogleConfig {
            client_id: "googleClientId".parse().unwrap(),
            client_secret: "googleClientSecret".parse().unwrap(),
            redirect_uri: "http://www.example.com".to_owned(),
            auth_uri: None,
            token_uri: None,
        });

        let result = sut.start_authentication("someState");

        check!(result == "https://accounts.google.com/o/oauth2/v2/auth?client_id=googleClientId&response_type=code&scope=openid%20email%20profile&redirect_uri=http%3A%2F%2Fwww.example.com&state=someState");
    }

    #[test]
    fn test_start_auth_custom_url() {
        let sut = GoogleAuthenticator::new(GoogleConfig {
            client_id: "googleClientId".parse().unwrap(),
            client_secret: "googleClientSecret".parse().unwrap(),
            redirect_uri: "http://www.example.com".to_owned(),
            auth_uri: Some("http://localhost:8000/google/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_owned()),
            token_uri: None,
        });

        let result = sut.start_authentication("someState");

        check!(result == "http://localhost:8000/google/o/oauth2/v2/auth?client_id=googleClientId&response_type=code&scope=openid%20email%20profile&redirect_uri=http%3A%2F%2Fwww.example.com&state=someState");
    }
}
