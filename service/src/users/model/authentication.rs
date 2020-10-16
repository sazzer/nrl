use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// The ID of an authentication provider.
#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ProviderID(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ProviderIDParseError {
    #[error("The provider ID must not be blank")]
    Blank,
}

impl FromStr for ProviderID {
    type Err = ProviderIDParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(Self::Err::Blank),
            value => Ok(Self(value.to_owned())),
        }
    }
}

/// The ID of a user with an authentication provider.
#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ProviderUserID(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ProviderUserIDParseError {
    #[error("The User ID must not be blank")]
    Blank,
}

impl FromStr for ProviderUserID {
    type Err = ProviderUserIDParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(Self::Err::Blank),
            value => Ok(Self(value.to_owned())),
        }
    }
}

/// Representation of the authentication details for a user with an external provider
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Authentication {
    pub provider: ProviderID,
    pub user: ProviderUserID,
    pub display_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_parse_providerid_success() {
        check!(Ok(ProviderID("hello".to_owned())) == "hello".parse());
        check!(Ok(ProviderID("hello".to_owned())) == "  hello".parse());
        check!(Ok(ProviderID("hello".to_owned())) == "hello  ".parse());
        check!(Ok(ProviderID("hello".to_owned())) == "  hello  ".parse());
    }

    #[test]
    fn test_parse_providerid_blank() {
        check!(Err(ProviderIDParseError::Blank) == "".parse::<ProviderID>());
        check!(Err(ProviderIDParseError::Blank) == "  ".parse::<ProviderID>());
    }

    #[test]
    fn test_parse_provideruserid_success() {
        check!(Ok(ProviderUserID("hello".to_owned())) == "hello".parse());
        check!(Ok(ProviderUserID("hello".to_owned())) == "  hello".parse());
        check!(Ok(ProviderUserID("hello".to_owned())) == "hello  ".parse());
        check!(Ok(ProviderUserID("hello".to_owned())) == "  hello  ".parse());
    }

    #[test]
    fn test_parse_provideruserid_blank() {
        check!(Err(ProviderUserIDParseError::Blank) == "".parse::<ProviderUserID>());
        check!(Err(ProviderUserIDParseError::Blank) == "  ".parse::<ProviderUserID>());
    }
}
