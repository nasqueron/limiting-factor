//! # API guards
//!
//! This module provides reusable guards to use with Rocket.

use rocket::data::{FromDataSimple, Outcome};
use rocket::{Data, Request};
use rocket::http::Status;
use rocket::Outcome::{Failure, Success};

use std::io::Read;

use limiting_factor_core::api::guards::{RequestBody, REQUEST_BODY_LIMIT};

// New-type wrapper for Rocket-specific implementations
#[derive(Debug, Clone)]
pub struct RocketRequestBody(pub RequestBody);

impl RocketRequestBody {
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

const ROCKET_REQUEST_BODY_LIMIT: u64 = REQUEST_BODY_LIMIT as u64;

impl FromDataSimple for RocketRequestBody {
    type Error = String;

    fn from_data(_request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut content = String::new();

        if let Err(e) = data.open().take(ROCKET_REQUEST_BODY_LIMIT).read_to_string(&mut content) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(Self(RequestBody { content }))
    }
}
