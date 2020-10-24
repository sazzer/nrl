use crate::authentication::{
    AuthenticationService, AuthenticatorID, StartAuthenticationError, StartAuthenticationUseCase,
};
use crate::http::problem::{Problem, NOT_FOUND};
use actix_web::web::{Data, HttpResponse, Path};
use std::sync::Arc;

/// HTTP Handler for listing the authenticators we can use.
///
/// # Parameters
/// - `authentication_service` - The authentication service.
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication/:id"),
    skip(authentication_service)
)]
pub async fn start_authentication(
    authentication_service: Data<Arc<AuthenticationService>>,
    path: Path<String>,
) -> Result<HttpResponse, Problem> {
    let authenticator: AuthenticatorID = path.0.parse().map_err(|_| Problem::from(NOT_FOUND))?;

    let result = authentication_service.start(authenticator).map_err(|e| {
        let problem = match e {
            StartAuthenticationError::UnknownAuthenticator => NOT_FOUND,
        };
        Problem::from(problem)
    })?;

    Ok(HttpResponse::Found()
        .header("Location", result.redirect_uri)
        .finish())
}
