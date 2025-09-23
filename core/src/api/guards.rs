/*  -------------------------------------------------------------
    Limiting Factor :: Core :: API :: Guards
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

//! # API guards
//!
//! This module provides common element for:
//!   - reusable guards to use with Rocket
//!   - reusable extractors to use with axum

use serde::{Deserialize, Serialize};

/// The maximum number of characters to read, to avoid DoS
pub const REQUEST_BODY_LIMIT: usize = 1_000_000;

/// A String representation of the request body. Useful when you need to pass it through as is.
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord)]
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
