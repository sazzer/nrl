use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::str::FromStr;

/// The display name of the user.
#[derive(Debug, PartialEq, FromSql)]
pub struct DisplayName(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DisplayNameParseError {
    #[error("The display name must not be blank")]
    Blank,
}

impl FromStr for DisplayName {
    type Err = DisplayNameParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(Self::Err::Blank),
            value => Ok(Self(value.to_owned())),
        }
    }
}

impl ToSql for DisplayName {
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
        check!(Ok(DisplayName("hello".to_owned())) == "hello".parse());
        check!(Ok(DisplayName("hello".to_owned())) == "  hello".parse());
        check!(Ok(DisplayName("hello".to_owned())) == "hello  ".parse());
        check!(Ok(DisplayName("hello".to_owned())) == "  hello  ".parse());
    }

    #[test]
    fn test_parse_blank() {
        check!(Err(DisplayNameParseError::Blank) == "".parse::<DisplayName>());
        check!(Err(DisplayNameParseError::Blank) == "  ".parse::<DisplayName>());
    }
}
