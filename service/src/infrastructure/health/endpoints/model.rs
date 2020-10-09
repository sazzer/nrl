use crate::infrastructure::health::{ComponentHealth, SystemHealth};
use actix_http::{http::StatusCode, Error, Response};
use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ok, Ready};
use serde::Serialize;
use std::collections::HashMap;

/// The HTTP representation of the health of a single component.
#[derive(Debug, Serialize)]
pub struct ComponentHealthModel {
    pub healthy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// The HTTP representation of the health of the entire system.
#[derive(Debug, Serialize)]
pub struct SystemHealthModel {
    pub healthy: bool,
    pub components: HashMap<String, ComponentHealthModel>,
}

impl From<ComponentHealth> for ComponentHealthModel {
    fn from(input: ComponentHealth) -> Self {
        Self {
            healthy: input.healthy(),
            message: input.message(),
        }
    }
}

impl From<SystemHealth> for SystemHealthModel {
    fn from(input: SystemHealth) -> Self {
        let healthy = input.healthy();
        let components = input
            .components
            .into_iter()
            .map(|(key, health)| (key, health.into()))
            .collect();

        Self {
            healthy,
            components,
        }
    }
}

impl Responder for SystemHealthModel {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    /// Generate an HTTP Response for the System Health.
    ///
    /// The status code is either `StatusCode::OK` for a healthy system or `StatusCode::SERVICE_UNAVAILABLE`
    /// for an unhealthy system, and the body is the JSON version of this type.
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let status_code = if self.healthy {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        let response = HttpResponse::build(status_code).json(self);

        ok(response)
    }
}
