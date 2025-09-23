//! # API guards
//!
//! This module provides reusable guards to use with Rocket.

use rocket::data::{FromDataSimple, Outcome};
use rocket::{Data, Request};
use rocket::http::Status;
use rocket::Outcome::{Failure, Success};
use serde::{Deserialize, Serialize};

use std::io::Read;

/// The maximum number of characters to read, to avoid DoS
const REQUEST_BODY_LIMIT: u64 = 1_000_000;

/// A String representation of the request body. Useful when you need to pass it through as is.
#[derive(Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord)]
pub struct RequestBody {
    /// The UTF-8 content of the request body
    pub content: String,
}

impl RequestBody {
    pub fn new () -> Self {
        Self {
            content: String::new(),
        }
    }

    /// Convert the request body into a string
    pub fn into_string (self) -> String {
        self.content
    }

    /// Convert the request body into a string, or None if it's empty
    pub fn into_optional_string (self) -> Option<String> {
        if self.content.is_empty() {
            None
        } else {
            Some(self.content)
        }
    }
}

impl FromDataSimple for RequestBody {
    type Error = String;

    fn from_data(_request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut content = String::new();

        if let Err(e) = data.open().take(REQUEST_BODY_LIMIT).read_to_string(&mut content) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(Self { content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_body_new () {
        let body = RequestBody::new();
        assert_eq!(0, body.content.len(), "Content should be empty");
    }

    #[test]
    fn test_request_body_into_string () {
        let body = RequestBody { content: "quux".to_string() };
        assert_eq!(String::from("quux"), body.into_string());
    }

    #[test]
    fn test_request_body_into_string_when_empty () {
        let body = RequestBody::new();
        assert_eq!(String::new(), body.into_string(), "Content should be empty");
    }

    #[test]
    fn test_request_body_into_optional_string () {
        let body = RequestBody { content: "quux".to_string() };
        assert_eq!(Some(String::from("quux")), body.into_optional_string());
    }

    #[test]
    fn test_request_body_into_optional_string_when_empty () {
        let body = RequestBody::new();
        assert_eq!(None, body.into_optional_string());
    }
}
