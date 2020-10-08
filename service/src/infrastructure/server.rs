use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::ServiceConfig, App, HttpServer};
use std::{ops::Deref, sync::Arc};

/// A function that is able to contribute configuration to the Actix server when it is being constructed.
pub type ServerConfig = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;

/// Wrapper around the HTTP Server.
pub struct Server {
    /// The set of configuration traits for the HTTP server.
    configs: Vec<ServerConfig>,
}

impl Server {
    /// Create a new instance of the server
    ///
    /// # Parameters
    /// - `configs` - The set of configuration lambdas that can contribute to the HTTP Server.
    ///
    /// # Returns
    /// The wrapper around the HTTP Server.
    #[must_use]
    pub fn new(configs: Vec<ServerConfig>) -> Self {
        Self { configs }
    }

    /// Start the HTTP Server listening on the given port number.
    ///
    /// # Parameters
    /// - `port` - The port to listen on.
    pub async fn start(self, port: u16) {
        let address = format!("0.0.0.0:{}", port);
        tracing::info!(address = ?address, "Starting web server");

        let configs = self.configs.clone();
        HttpServer::new(move || {
            let configs = configs.clone();

            let mut app = App::new()
                .wrap(Logger::default())
                .wrap(Cors::new().expose_headers(vec![header::ETAG]).finish());

            for config in &configs {
                app = app.configure(config.deref());
            }

            app
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
