#[actix_rt::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Starting NRL");

    nrl_lib::main().await
}
