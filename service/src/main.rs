use config::{Config, Environment};
use serde::Deserialize;

/// Representation of the application settings that will be loaded from the environment
#[derive(Debug, Deserialize)]
struct Settings {
    /// The port on which the HTTP server should listen on
    pub port: Option<u16>,

    /// The URL to connect to the database with
    pub database_url: String,

    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_auth_uri: Option<String>,
    pub google_token_uri: Option<String>,
    pub google_redirect_uri: Option<String>,
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
        let google_config = match (
            self.google_client_id,
            self.google_client_secret,
            self.google_redirect_uri,
        ) {
            (Some(client_id), Some(client_secret), Some(redirect_uri)) => {
                Some(nrl_lib::GoogleConfig {
                    client_id: client_id.parse().expect("Failed to parse Google Client ID"),
                    client_secret: client_secret
                        .parse()
                        .expect("Failed to parse Google Client Secret"),
                    auth_uri: self.google_auth_uri,
                    token_uri: self.google_token_uri,
                    redirect_uri,
                })
            }
            _ => None,
        };

        nrl_lib::ServiceSettings {
            database_url: self.database_url,
            google_config,
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
    let service = nrl_lib::Service::new(settings.into()).await;

    service.start(port).await;
}
