use actix_http::Request;
use nrl_lib::{Service, ServiceSettings, TestResponse};

pub struct TestServer {
    service: Service,
}

impl TestServer {
    pub async fn new() -> Self {
        tracing_subscriber::fmt::init();

        let settings = ServiceSettings {};
        let service = Service::new(&settings);

        Self { service }
    }

    /// Inject a request into the server. Only used for testing
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
