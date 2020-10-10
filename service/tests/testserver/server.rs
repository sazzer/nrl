use super::testdatabase::TestDatabase;
use actix_http::Request;
use nrl_lib::{Service, ServiceSettings, TestResponse};

/// The test server with which to perform integration tests.
pub struct TestServer {
    service: Service,
    #[allow(dead_code)]
    database: TestDatabase,
}

impl TestServer {
    /// Create a new instance of the test server.
    pub async fn new() -> Self {
        tracing_subscriber::fmt::init();

        let database = TestDatabase::default();

        let settings = ServiceSettings {
            database_url: database.url.clone(),
        };
        let service = Service::new(&settings);

        Self { service, database }
    }

    /// Inject a request into the server. Only used for testing
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
