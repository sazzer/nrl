use crate::authentication::{
    AuthenticationService, AuthenticatorID, CompleteAuthenticationUseCase,
};
use crate::http::problem::{Problem, NOT_FOUND};
use actix_web::web::{Data, HttpResponse, Path, Query};
use std::{collections::HashMap, sync::Arc};

/// HTTP Handler for listing the authenticators we can use.
///
/// # Parameters
/// - `authentication_service` - The authentication service.
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication/:id/complete"),
    skip(authentication_service)
)]
pub async fn complete_authentication(
    authentication_service: Data<Arc<AuthenticationService>>,
    params: Query<HashMap<String, String>>,
    path: Path<String>,
) -> Result<HttpResponse, Problem> {
    let authenticator: AuthenticatorID = path.0.parse().map_err(|_| Problem::from(NOT_FOUND))?;

    let (user, security_context, signed_security_context) = authentication_service
        .complete(authenticator, params.0)
        .await
        .unwrap();

    tracing::info!(user = ?user, security_context = ?security_context, signed_security_context = ?signed_security_context, "Authenticated");

    Ok(HttpResponse::Ok().finish())
}
