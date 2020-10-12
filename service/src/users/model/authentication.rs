/// The ID of an authentication provider.
#[derive(Debug, PartialEq)]
pub struct ProviderID(String);

/// The ID of a user with an authentication provider.
#[derive(Debug, PartialEq)]
pub struct ProviderUserID(String);

/// Representation of the authentication details for a user with an external provider
#[derive(Debug, PartialEq)]
pub struct Authentication {
    provider: ProviderID,
    user: ProviderUserID,
    display_name: String,
}
