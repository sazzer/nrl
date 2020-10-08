use actix_web::Responder;

/// HTTP Handler for checking the health of the system.
///
/// # Parameters
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(fields(http_method = "GET", http_path = "/health"), skip())]
pub async fn get_health() -> impl Responder {
    "Hello".to_owned()
}
