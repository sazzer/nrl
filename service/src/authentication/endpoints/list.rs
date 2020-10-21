use crate::authentication::{AuthenticationService, AuthenticatorID, ListProvidersUseCase};
use crate::http::page::PageApiModel;
use actix_web::web::Data;
use std::sync::Arc;

/// HTTP Handler for listing the authenticators we can use.
///
/// # Parameters
/// - `authentication_service` - The authentication service.
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication"),
    skip(authentication_service)
)]
pub async fn list_authenticators(
    authentication_service: Data<Arc<AuthenticationService>>,
) -> PageApiModel<AuthenticatorID> {
    authentication_service
        .list()
        .into_iter()
        .cloned()
        .collect::<Vec<AuthenticatorID>>()
        .into()
}
