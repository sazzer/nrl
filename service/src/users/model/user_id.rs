use uuid::Uuid;

/// The actual ID of the user resource.
#[derive(Debug, PartialEq)]
pub struct UserID(Uuid);

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
