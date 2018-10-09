//! # API standard and JSON responses.
//!
//! This module provides useful traits and methods to craft API replies from an existing type.

#[cfg(feature = "pgsql")]
use diesel::result::{DatabaseErrorInformation, DatabaseErrorKind, QueryResult};
#[cfg(feature = "pgsql")]
use diesel::result::Error as ResultError;

use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;

#[cfg(feature = "pgsql")]
use std::error::Error;

/*   -------------------------------------------------------------
     Custom types
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub type ApiJsonResponse<T> = Result<Json<T>, Failure>;

/*   -------------------------------------------------------------
     API Response

     :: Implementation for QueryResult (Diesel ORM)
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This trait allows to consume an object into an HTTP response.
pub trait ApiResponse<T> {
    /// Consumes the value and creates a JSON or a Failure result response.
    fn into_json_response(self) -> ApiJsonResponse<T>;
}

#[cfg(feature = "pgsql")]
impl<T> ApiResponse<T> for QueryResult<T> {
    /// Prepares an API response from a query result.
    ///
    /// The result is the data structure prepared by the Diesel ORM after a SELECT query
    /// with one result, for example using `first` method. You can also you use it to
    /// parse the returning result (... RETURNING *), which is a default for Diesel after
    /// an INSERT query.
    ///
    /// So result can be:
    ///   - Ok(T)
    ///   - Err(any database error)
    ///
    /// # Examples
    ///
    /// To offer a /player/foo route to serve player information from the player table:
    ///
    /// ```
    /// use limiting_factor::api::ApiResponse;
    /// use limiting_factor::api::ApiJsonResponse;
    ///
    /// #[get("/player/<name>")]
    /// pub fn get_player(connection: DatabaseConnection, name: String) -> ApiJsonResponse<Player> {
    ///     players
    ///         .filter(username.eq(&name))
    ///         .first::<Player>(&*connection)
    ///         .into_json_response()
    /// }
    /// ```
    ///
    /// This will produce a JSON representation when the result is found,
    /// a 404 error when no result is found, a 500 error if there is a database issue.
    fn into_json_response(self) -> ApiJsonResponse<T> {
        match self {
            // CASE I - The query returns one value, we return a JSON representation fo the item
            Ok(item) => Ok(Json(item)),

            Err(error) => match error {
                // Case II - The query returns no result, we return a 404 Not found response
                ResultError::NotFound => Err(Failure::from(Status::NotFound)),

                // Case III - We need to handle a database error, which could be a 400/409/500
                ResultError::DatabaseError(kind, details) => Err(build_database_error_response(kind, details)),

                // Case IV - The error is probably server responsbility, log it and throw a 500
                _ => Err(error.into_failure_response()),
            }
        }
    }
}

/*   -------------------------------------------------------------
     Failure response

     :: Implementation for diesel::result::Error
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This trait allows to consume an object into an HTTP failure response.
pub trait FailureResponse {
    /// Consumes the variable and creates a Failure response .
    fn into_failure_response(self) -> Failure;
}

#[cfg(feature = "pgsql")]
impl FailureResponse for ResultError {
    /// Consumes the error and creates a Failure 500 Internal server error response.
    fn into_failure_response(self) -> Failure {
        build_internal_server_error_response(self.description())
    }
}

/*   -------------------------------------------------------------
     Helper methods to prepare API responses
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub fn build_internal_server_error_response(message: &str) -> Failure {
    warn!(target:"api", "{}", message);

    Failure::from(Status::InternalServerError)
}

#[cfg(feature = "pgsql")]
fn build_database_error_response(error_kind: DatabaseErrorKind, info: Box<dyn DatabaseErrorInformation>) -> Failure {
    match error_kind {
        // Case IIIa - The query tries to do an INSERT violating an unique constraint
        //             e.g. two INSERT with the same unique value
        //             We return a 409 Conflict
        DatabaseErrorKind::UniqueViolation => Failure::from(Status::Conflict),

        // Case IIIb - The query violated a foreign key constraint
        //             e.g. an INSERT referring to a non existing user 1004
        //                  when there is no id 1004 in users table
        //             We return a 400 Bad request
        DatabaseErrorKind::ForeignKeyViolation => Failure::from(Status::BadRequest),

        // Case IIIc - For other databases errors, the client responsibility isn't involved.
        _ => build_internal_server_error_response(info.message()),
    }
}
