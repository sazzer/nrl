use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;

/// The username of the user.
#[derive(Debug, PartialEq, FromSql, Serialize)]
pub struct Username(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UsernameParseError {
    #[error("The username must not be blank")]
    Blank,
}

impl FromStr for Username {
    type Err = UsernameParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(Self::Err::Blank),
            value => Ok(Self(value.to_owned())),
        }
    }
}

impl ToSql for Username {
    accepts!(TEXT, VARCHAR);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_parse_success() {
        check!(Ok(Username("hello".to_owned())) == "hello".parse());
        check!(Ok(Username("hello".to_owned())) == "  hello".parse());
        check!(Ok(Username("hello".to_owned())) == "hello  ".parse());
        check!(Ok(Username("hello".to_owned())) == "  hello  ".parse());
    }

    #[test]
    fn test_parse_blank() {
        check!(Err(UsernameParseError::Blank) == "".parse::<Username>());
        check!(Err(UsernameParseError::Blank) == "  ".parse::<Username>());
    }
}
