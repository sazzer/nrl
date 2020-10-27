use crate::users::{ProviderID, ProviderIDParseError};
use serde::Serialize;
use std::convert::TryInto;
use std::str::FromStr;

/// The ID of an authenticator.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub struct AuthenticatorID(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthenticatorIDParseError {
    #[error("The ID must not be blank")]
    Blank,
}

impl FromStr for AuthenticatorID {
    type Err = AuthenticatorIDParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(Self::Err::Blank),
            value => Ok(Self(value.to_owned())),
        }
    }
}

impl TryInto<ProviderID> for AuthenticatorID {
    type Error = ProviderIDParseError;

    fn try_into(self) -> Result<ProviderID, ProviderIDParseError> {
        self.0.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_parse_success() {
        check!(Ok(AuthenticatorID("hello".to_owned())) == "hello".parse());
        check!(Ok(AuthenticatorID("hello".to_owned())) == "  hello".parse());
        check!(Ok(AuthenticatorID("hello".to_owned())) == "hello  ".parse());
        check!(Ok(AuthenticatorID("hello".to_owned())) == "  hello  ".parse());
    }

    #[test]
    fn test_parse_blank() {
        check!(Err(AuthenticatorIDParseError::Blank) == "".parse::<AuthenticatorID>());
        check!(Err(AuthenticatorIDParseError::Blank) == "  ".parse::<AuthenticatorID>());
    }
}
