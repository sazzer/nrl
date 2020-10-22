use std::str::FromStr;

/// The Client ID for authenticating with Google.
#[derive(Debug)]
pub struct GoogleClientId(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum GoogleClientIdParseError {
    #[error("The Client ID must not be blank")]
    Blank,
}

impl FromStr for GoogleClientId {
    type Err = GoogleClientIdParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(Self::Err::Blank),
            value => Ok(Self(value.to_owned())),
        }
    }
}

/// The Client Secret for authenticating with Google.
#[derive(Debug)]
pub struct GoogleClientSecret(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum GoogleClientSecretParseError {
    #[error("The Client Secret must not be blank")]
    Blank,
}

impl FromStr for GoogleClientSecret {
    type Err = GoogleClientSecretParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(Self::Err::Blank),
            value => Ok(Self(value.to_owned())),
        }
    }
}

/// Configuration for the Google authenticator
#[derive(Debug)]
pub struct GoogleConfig {
    pub client_id: GoogleClientId,
    pub client_secret: GoogleClientSecret,
    pub auth_uri: Option<String>,
    pub token_uri: Option<String>,
    pub redirect_uri: String,
}
