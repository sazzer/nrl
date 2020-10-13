use super::{Authentication, DisplayName, Email, UserID, Username};
use crate::model::Model;

/// The data to represent a single user.
#[derive(Debug)]
pub struct UserData {
    pub username: Option<Username>,
    pub email: Option<Email>,
    pub display_name: DisplayName,
    pub authentications: Vec<Authentication>,
}

/// API Model to represent a single user.
pub type UserModel = Model<UserID, UserData>;
