use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::str::FromStr;

/// The email address of the user.
#[derive(Debug, PartialEq, FromSql)]
pub struct Email(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EmailParseError {
    #[error("The email must not be blank")]
    Blank,
}

impl FromStr for Email {
    type Err = EmailParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "" => Err(EmailParseError::Blank),
            value => Ok(Email(value.to_owned())),
        }
    }
}

impl ToSql for Email {
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
        check!(Ok(Email("hello".to_owned())) == "hello".parse());
        check!(Ok(Email("hello".to_owned())) == "  hello".parse());
        check!(Ok(Email("hello".to_owned())) == "hello  ".parse());
        check!(Ok(Email("hello".to_owned())) == "  hello  ".parse());
    }

    #[test]
    fn test_parse_blank() {
        check!(Err(EmailParseError::Blank) == "".parse::<Email>());
        check!(Err(EmailParseError::Blank) == "  ".parse::<Email>());
    }
}
