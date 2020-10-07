mod infrastructure;

pub async fn main() {
    let service = infrastructure::service::Service::new();

    service.start(8000).await;
}
