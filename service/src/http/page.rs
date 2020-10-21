use actix_http::{http::StatusCode, Error, Response};
use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ok, Ready};
use serde::Serialize;

/// HTTP API Model to represent pagination details for a page of results.
#[derive(Debug, Serialize)]
pub struct PaginationApiModel {
    pub offset: usize,
    pub total: usize,
}

/// HTTP API model representing a page of results.
#[derive(Debug, Serialize)]
pub struct PageApiModel<T> {
    pub entries: Vec<T>,
    pub pagination: PaginationApiModel,
}

impl<T> PageApiModel<T> {
    /// Create a new HTTP API Model for a page of results.
    pub fn new(entries: Vec<T>, offset: usize, total: usize) -> Self {
        Self {
            entries,
            pagination: PaginationApiModel { offset, total },
        }
    }
}

impl<T> From<Vec<T>> for PageApiModel<T> {
    fn from(entries: Vec<T>) -> Self {
        let length = entries.len();
        Self::new(entries, 0, length)
    }
}

impl<T> Responder for PageApiModel<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    /// Generate an HTTP Response for the Page of results.
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let response = HttpResponse::build(StatusCode::OK).json(self);

        ok(response)
    }
}
