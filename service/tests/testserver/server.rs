use actix_http::Request;
use actix_web::{
    error::ParseError,
    http::header::{
        Header, HeaderName, HeaderValue, IntoHeaderValue, InvalidHeaderValue, AUTHORIZATION,
    },
};
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
            google_config: None,
        };
        let service = Service::new(settings).await;

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

    pub fn authenticate<S>(&self, user_id: S) -> impl Header
    where
        S: Into<String>,
    {
        let security_context = self
            .service
            .generate_security_context(&user_id.into().parse().unwrap());

        Authorization(security_context.to_string())
    }
}

/// Representation of the Authorization header containing a signed security context
struct Authorization(String);

impl IntoHeaderValue for Authorization {
    type Error = InvalidHeaderValue;

    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        let value = format!("Bearer {}", self.0);
        HeaderValue::from_str(&value)
    }
}
impl Header for Authorization {
    fn name() -> HeaderName {
        AUTHORIZATION
    }

    fn parse<T>(_: &T) -> Result<Self, ParseError> {
        todo!()
    }
}
