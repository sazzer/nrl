use std::convert::AsRef;
use std::str::FromStr;
use uritemplate::{IntoTemplateVar, TemplateVar};

/// The Client ID for authenticating with Google.
#[derive(Debug, Clone)]
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

impl<'a> IntoTemplateVar for GoogleClientId {
    fn into_template_var(self) -> TemplateVar {
        TemplateVar::Scalar(self.0)
    }
}

impl AsRef<str> for GoogleClientId {
    fn as_ref(&self) -> &str {
        &self.0
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

impl AsRef<str> for GoogleClientSecret {
    fn as_ref(&self) -> &str {
        &self.0
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
