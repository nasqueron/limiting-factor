/*  -------------------------------------------------------------
    Limiting Factor :: axum :: API :: replies
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

//! # API standard and JSON responses.
//!
//! This module provides useful traits and methods to craft API replies from an existing type.

use axum::http::StatusCode;
use axum::Json;

#[cfg(feature = "serialization")]
use serde::Serialize;

/*  -------------------------------------------------------------
    JSON responses
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

pub type ApiJsonResponse<T> = Result<Json<T>, (StatusCode, Json<String>)>;

/// This trait allows to consume an object into an HTTP response.
pub trait ApiResponse<T> {
    /// Consumes the value and creates a JSON or a Status result response.
    fn into_json_response(self) -> ApiJsonResponse<T>;
}

impl<T> ApiResponse<T> for Json<T> {
    fn into_json_response(self) -> ApiJsonResponse<T> {
        Ok(self)
    }
}

#[cfg(feature = "serialization")]
impl<T> ApiResponse<T> for T where T: Serialize {
    fn into_json_response(self) -> ApiJsonResponse<T> {
        Ok(Json(self))
    }
}

//  -------------------------------------------------------------
//  Failures
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait FailureResponse {
    fn status_code(&self) -> StatusCode;

    fn response(&self) -> String;
}

impl<T, E> ApiResponse<T> for Result<T, E>
    where T: ApiResponse<T>, E: FailureResponse
{
    fn into_json_response(self) -> ApiJsonResponse<T> {
        match self {
            Ok(value) => value.into_json_response(),
            Err(error) => Err((error.status_code(), Json(error.response())))
        }
    }
}
