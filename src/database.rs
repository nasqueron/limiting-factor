//! This module handles a database layer, mainly intended to be used
//! with a web server or framework like Rocket or Iron.
//!
//! It leverages diesel and r2d2.
//!
//! Most code comes from the Rocket manual:
//! https://rocket.rs/guide/state/#databases

use diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use ErrorResult;
use r2d2::Error as PoolError;
use rocket::http::Status;
use rocket::Outcome;
use rocket::request::FromRequest;
use rocket::request::Outcome as RequestOutcome;
use rocket::Request;
use rocket::State;
use std::ops::Deref;

/*   -------------------------------------------------------------
     Custom types
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub type PostgreSQLPool = Pool<ConnectionManager<PgConnection>>;

/*   -------------------------------------------------------------
     DatabaseConnection

     :: FromRequest
     :: Deref
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Represents an established working database connection from the pool
pub struct DatabaseConnection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DatabaseConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> RequestOutcome<Self, Self::Error> {
        let pool = request.guard::<State<PostgreSQLPool>>()?;
        match pool.get() {
            Ok(connection) => Outcome::Success(DatabaseConnection(connection)),
            Err(error) => {
                warn!(target:"request", "Can't get a connection from the pool: {}", error);

                Outcome::Failure((Status::ServiceUnavailable, ()))
            },
        }
    }
}

impl Deref for DatabaseConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*   -------------------------------------------------------------
     Helper methods to get a database connection
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Builds a r2d2 database pool, to be used in a request guard or a managed state.
///
/// # Examples
///
/// ```
/// rocket::ignite()
///    .manage(initialize_database_pool(String::from("postgres://::1/test"), 4)?)
///    .mount("/", routes)
///    .launch();
/// ```
pub fn initialize_database_pool(url: &str, max_size: u32) -> Result<PostgreSQLPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .max_size(max_size)
        .build(manager)
}

/// Allows to test if it's possible to establish a connection to the database.
///
/// The goal is to test early any issue with the connection, and loudly warn or fail
/// if the database can't be reached.
///
/// # Examples
///
/// ```
/// // Initial connection to test if the database configuration works
/// {
///     test_database_connection(&config.database_url)?;
///     info!(target: "runner", "Connection to database established.");
/// }
/// ```
pub fn test_database_connection(database_url: &str) -> ErrorResult<()> {
    PgConnection::establish(database_url)?;

    Ok(())
}
