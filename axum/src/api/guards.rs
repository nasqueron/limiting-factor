/*  -------------------------------------------------------------
    Limiting Factor :: axum :: API :: Guards
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

//! # API extractors
//!
//! This module provides reusable extractors to use with axum.

use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;

use limiting_factor_core::api::guards::{RequestBody, REQUEST_BODY_LIMIT};

// New-type wrapper for Axum-specific implementations
#[derive(Debug, Clone)]
pub struct AxumRequestBody(pub RequestBody);

impl AxumRequestBody {
    pub fn new() -> Self {
        Self(RequestBody::new())
    }

    // Delegate methods
    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    pub fn into_optional_string(self) -> Option<String> {
        self.0.into_optional_string()
    }
}

impl From<RequestBody> for AxumRequestBody {
    fn from(data: RequestBody) -> Self {
        Self(data)
    }
}

impl From<AxumRequestBody> for RequestBody {
    fn from(body: AxumRequestBody) -> Self {
        body.0
    }
}

/// Error type during a request body extraction
#[derive(Debug)]
pub enum RequestBodyError {
    /// Body size is greater than REQUEST_BODY_LIMIT (DoS risk)
    TooLarge,

    /// Not in UTF-8 encoding
    InvalidEncoding,

    /// I/O error
    ReadError(String),
}

impl IntoResponse for RequestBodyError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RequestBodyError::TooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "Request body too large".to_string(),
            ),

            RequestBodyError::InvalidEncoding => (
                StatusCode::BAD_REQUEST,
                "Request body contains invalid characters when trying to decode as UTF-8".to_string(),
            ),

            RequestBodyError::ReadError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read request body: {}", err),
            ),
        };

        (status, message).into_response()
    }
}

impl<S> FromRequest<S> for AxumRequestBody
where
    S: Send + Sync,
{
    type Rejection = RequestBodyError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the body from the request
        let body = req.into_body();

        // Collect the body with size limit
        let collected = match body.collect().await {
            Ok(collected) => collected,
            Err(e) => return Err(RequestBodyError::ReadError(e.to_string())),
        };

        let bytes = collected.to_bytes();

        // Check size limit
        if bytes.len() > REQUEST_BODY_LIMIT {
            return Err(RequestBodyError::TooLarge);
        }

        // Convert to UTF-8 string
        let content = match String::from_utf8(bytes.to_vec()) {
            Ok(content) => content,
            Err(_) => return Err(RequestBodyError::InvalidEncoding),
        };

        Ok(Self(RequestBody { content }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_body_extraction() {
        use axum::body::Body;
        use axum::http::Request;

        let req = Request::builder()
            .body(Body::empty())
            .unwrap();

        let body = AxumRequestBody::from_request(req, &()).await.unwrap();
        assert_eq!("", body.0.content);
        assert_eq!(None, body.into_optional_string());
    }

    #[tokio::test]
    async fn test_body_extraction() {
        use axum::body::Body;
        use axum::http::Request;

        let req = Request::builder()
            .body(Body::from("lorem ipsum dolor"))
            .unwrap();

        let body = AxumRequestBody::from_request(req, &()).await.unwrap();
        assert_eq!("lorem ipsum dolor", body.0.content);
        assert_eq!(Some("lorem ipsum dolor".to_string()), body.into_optional_string());
    }
}
