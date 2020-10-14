use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use uuid::Uuid;

/// The actual ID of the user resource.
#[derive(Debug, PartialEq, FromSql)]
pub struct UserID(Uuid);

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for UserID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl ToSql for UserID {
    accepts!(UUID);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}
