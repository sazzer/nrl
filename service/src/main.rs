use config::{Config, Environment};
use serde::Deserialize;

/// Representation of the application settings that will be loaded from the environment
#[derive(Debug, Deserialize)]
struct Settings {
    /// The port on which the HTTP server should listen on
    pub port: Option<u16>,

    /// The URL to connect to the database with
    pub database_url: String,
}

impl Default for Settings {
    /// Construct the settings from the environment
    ///
    /// # Returns
    /// The Settings object, loaded from the environment variables
    fn default() -> Self {
        let mut s = Config::new();
        s.merge(Environment::default())
            .expect("Failed to load environment properties");

        s.try_into().expect("Failed to build settings from config")
    }
}

impl Into<nrl_lib::ServiceSettings> for Settings {
    fn into(self) -> nrl_lib::ServiceSettings {
        nrl_lib::ServiceSettings {
            database_url: self.database_url,
        }
    }
}

#[actix_rt::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let settings = Settings::default();
    tracing::debug!(settings = ?settings, "Loaded settings");

    let port = settings.port.unwrap_or(8000);
    let service = nrl_lib::Service::new(&settings.into()).await;

    service.start(port).await;
}
