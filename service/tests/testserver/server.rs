use actix_http::Request;
use nrl_lib::{Service, ServiceSettings, TestResponse};
use nrl_testdatabase::{seeddata::SeedData, TestDatabase};

/// The test server with which to perform integration tests.
pub struct TestServer {
    service: Service,
    #[allow(dead_code)]
    database: TestDatabase,
}

impl TestServer {
    /// Create a new instance of the test server.
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let database = TestDatabase::default();

        let settings = ServiceSettings {
            database_url: database.url.clone(),
        };
        let service = Service::new(&settings).await;

        Self { service, database }
    }

    /// Inject a request into the server. Only used for testing
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }

    /// Seed some data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed
    pub async fn seed(&self, data: &dyn SeedData) -> &Self {
        self.database.seed(data).await;
        self
    }
}
