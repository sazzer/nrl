use super::service::Service;
use crate::authorization::SignedSecurityContext;
use crate::users::UserID;
use actix_http::Request;
use actix_web::App;
use std::ops::Deref;

impl Service {
    /// Inject a request into the server. Only used for testing
    pub async fn inject(&self, req: Request) -> TestResponse {
        let configs = self.server.configs.clone();

        let mut app = App::new();

        for config in &configs {
            app = app.configure(config.deref());
        }

        let mut test_service = actix_web::test::init_service(app).await;
        let response = actix_web::test::call_service(&mut test_service, req).await;

        let status = response.status();
        let headers = response.headers().clone();
        let body = actix_web::test::read_body(response).await;

        TestResponse {
            status,
            headers,
            body,
        }
    }

    /// Helper to generate a security context for the given User ID. Only used for testing.
    #[must_use]
    pub fn generate_security_context(&self, user_id: &UserID) -> SignedSecurityContext {
        self.generate_security_context_service
            .generate_security_context(user_id.into())
            .1
    }
}

/// Representation of the response to injecting a test request
pub struct TestResponse {
    /// The status code
    pub status: actix_http::http::StatusCode,
    /// The set of headers
    pub headers: actix_http::http::HeaderMap,
    /// The response body
    pub body: bytes::Bytes,
}

impl TestResponse {
    /// Get the value of the header with the given name
    ///
    /// # Parameters
    /// - `name` - The name of the header
    ///
    /// # Returns
    /// The header, if present. `None` if it wasn't present.
    pub fn header<S>(&self, name: S) -> Option<&actix_http::http::HeaderValue>
    where
        S: Into<String>,
    {
        self.headers.get(name.into())
    }

    /// Convert the response body to JSON
    ///
    /// # Returns
    /// The body of the response, converted to a Serde JSON object
    ///
    /// # Errors
    /// Any errors from deserializing the response
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::error::Error> {
        serde_json::from_slice(&self.body)
    }
}
