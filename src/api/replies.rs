//! # API standard and JSON responses.
//!
//! This module provides useful traits and methods to craft API replies from an existing type.

#[cfg(feature = "pgsql")]
use diesel::result::{DatabaseErrorInformation, DatabaseErrorKind, QueryResult};
#[cfg(feature = "pgsql")]
use diesel::result::Error as ResultError;

use rocket::http::Status;
use rocket_contrib::json::Json;

#[cfg(feature = "serialization")]
use serde::Serialize;

#[cfg(feature = "pgsql")]
use std::error::Error;

/*   -------------------------------------------------------------
     Custom types
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub type ApiJsonResponse<T> = Result<Json<T>, Status>;

/*   -------------------------------------------------------------
     API Response

     :: Implementation for QueryResult (Diesel ORM)
     :: Implementation for Json (Rocket contrib)
     :: Implementation for Serialize (Serde)
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This trait allows to consume an object into an HTTP response.
pub trait ApiResponse<T> {
    /// Consumes the value and creates a JSON or a Status result response.
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
    ///   - Err(E) where E is a Status containing an HTTP error code according the situation
    ///
    /// # Examples
    ///
    /// To offer a /player/foo route to serve player information from the players table:
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
    ///
    /// To insert a new player in the same table:
    ///
    /// ```
    /// use limiting_factor::api::ApiResponse;
    /// use limiting_factor::api::ApiJsonResponse;
    ///
    /// #[post("/register", format="application/json", data="<user>")]
    /// pub fn register(connection: DatabaseConnection,  user: Json<UserToRegister>) -> ApiJsonResponse<Player> {
    ///     let user: UserToRegister = user.into_inner();
    ///     let player_to_create = user.to_new_player();
    ///
    ///     diesel::insert_into(players)
    ///         .values(&player_to_create)
    ///         .get_result::<Player>(&*connection)
    ///         .into_json_response()
    /// }
    /// ```
    ///
    /// This will produce a JSON representation of the newly inserted player if successful.
    /// If the insert fails because of an unique constraint violation (e.g. an username already
    /// taken), it returns a 409 Conflict.
    /// If the failure is from a foreign key integrity constraint, it returns a 400.
    /// If there is any other database issue, it returns a 500.
    fn into_json_response(self) -> ApiJsonResponse<T> {
        self
            // CASE I - The query returns one value, we return a JSON representation fo the item
            .map(|item| Json(item))
            .map_err(|error| match error {
                // Case II - The query returns no result, we return a 404 Not found response
                ResultError::NotFound => Status::NotFound,

                // Case III -  We need to handle a database error, which could be a 400/409/500
                ResultError::DatabaseError(kind, details) => {
                    build_database_error_response(kind, details)
                }

                // Case IV - The error is probably server responsibility, log it and throw a 500
                _ => error.into_failure_response(),
            })
    }
}

/// Prepares an API response from a JSON.
impl<T> ApiResponse<T> for Json<T> {
    fn into_json_response(self) -> ApiJsonResponse<T> {
        Ok(self)
    }
}

/// Prepares an API response from a Serde-serializable result.
///
/// This is probably the easiest way to convert most struct
/// into API responders.
///
/// # Examples
///
#[cfg(feature = "serialization")]
impl<T> ApiResponse<T> for T
    where T: Serialize
{
    fn into_json_response(self) -> ApiJsonResponse<T> {
        Ok(Json(self))
    }
}

/*   -------------------------------------------------------------
     Failure response

     :: Implementation for diesel::result::Error
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This trait allows to consume an object into an HTTP failure response.
pub trait FailureResponse {
    /// Consumes the variable and creates a Failure response .
    fn into_failure_response(self) -> Status;
}

#[cfg(feature = "pgsql")]
impl FailureResponse for ResultError {
    /// Consumes the error and creates a 500 Internal server error Status response.
    fn into_failure_response(self) -> Status {
        build_internal_server_error_response(self.description())
    }
}

/*   -------------------------------------------------------------
     Helper methods to prepare API responses
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

#[deprecated(since="0.6.0", note="Use directly Status::NotFound instead.")]
pub fn build_not_found_response() -> Status {
    Status::NotFound
}

#[deprecated(since="0.6.0", note="Use directly Status::BadRequest instead.")]
pub fn build_bad_request_response() -> Status {
    Status::BadRequest
}

pub fn build_internal_server_error_response(message: &str) -> Status {
    warn!(target:"api", "{}", message);

    Status::InternalServerError
}

#[cfg(feature = "pgsql")]
fn build_database_error_response(error_kind: DatabaseErrorKind, info: Box<dyn DatabaseErrorInformation>) -> Status {
    match error_kind {
        // Case IIIa - The query tries to do an INSERT violating an unique constraint
        //             e.g. two INSERT with the same unique value
        //             We return a 409 Conflict
        DatabaseErrorKind::UniqueViolation => Status::Conflict,

        // Case IIIb - The query violated a foreign key constraint
        //             e.g. an INSERT referring to a non existing user 1004
        //                  when there is no id 1004 in users table
        //             We return a 400 Bad request
        DatabaseErrorKind::ForeignKeyViolation => Status::BadRequest,

        // Case IIIc - For other databases errors, the client responsibility isn't involved.
        _ => build_internal_server_error_response(info.message()),
    }
}
