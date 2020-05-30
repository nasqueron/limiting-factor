//! # Service configuration.
//!
//! This module allows to configure the service.
//!
//! It provides a Config trait to build custom configuration implementation.
//!
//! It also provides a `DefaultConfig` implementation of this `Config` trait to
//! extract variables from an .env file or environment.

use dotenv::dotenv;
#[cfg(feature = "pgsql")]
use kernel::DefaultService;
use kernel::{MinimalService, Service};
use rocket::Route;
use std::env;
use ErrorResult;

/*   -------------------------------------------------------------
     Config trait
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This trait allows to provide a configuration for the resources needed by the API.
pub trait Config {
    fn get_database_url(&self) -> &str;
    fn get_entry_point(&self) -> &str;
    fn get_database_pool_size(&self) -> u32;
    fn with_database(&self) -> bool;
    fn into_service(self, routes: Vec<Route>) -> Box<dyn Service>;
}

/*   -------------------------------------------------------------
     EnvironmentConfigurable
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This trait allows to configure the object from the environment
pub trait EnvironmentConfigurable {
    fn parse_environment() -> ErrorResult<Self> where Self: Sized;
}

/*   -------------------------------------------------------------
     DefaultConfig

     :: Config
     :: sui generis implementation
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This is a default implementation of the `Config` trait, which extracts  the following variables
/// from an .env file or environment:
///
///   - `API_ENTRY_POINT` (facultative, by default `/`): the mounting point of the API methods
///   - `DATABASE_URL` (mandatory): the URL to connect to your database
///   - `DATABASE_POOL_SIZE` (facultative, by default 4): the number of connections to open
#[cfg(feature = "pgsql")]
pub struct DefaultConfig {
    database_url: String,
    entry_point: String,
    database_pool_size: u32,
    with_database: bool,
}

#[cfg(feature = "pgsql")]
impl DefaultConfig {
    const DEFAULT_DATABASE_POOL_SIZE: u32 = 4;
}

#[cfg(feature = "pgsql")]
impl Config for DefaultConfig {
    fn get_database_url(&self) -> &str { &self.database_url }

    fn get_entry_point(&self) -> &str { &self.entry_point }

    fn get_database_pool_size(&self) -> u32 { self.database_pool_size }

    fn with_database(&self) -> bool { self.with_database }

    fn into_service(self, routes: Vec<Route>) -> Box<dyn Service> {
        let service = DefaultService {
            config: self,
            routes,
        };

        Box::new(service)
    }
}

#[cfg(feature = "pgsql")]
impl EnvironmentConfigurable for DefaultConfig {
    fn parse_environment() -> ErrorResult<Self> {
        if let Err(error) = dotenv() {
            warn!(target: "config", "Can't parse .env: {}", error);
        };

        let with_database = env::var("LF_DISABLE_DATABASE").is_err();

        let database_url = match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(e) => {
                if with_database {
                    error!(target: "config", "You need to specify a DATABASE_URL variable in the environment (or .env file).");
                    return Err(Box::new(e));
                }

                String::new()
            }
        };

        let entry_point = env::var("API_ENTRY_POINT").unwrap_or(String::from("/"));

        let database_pool_size = match env::var("DATABASE_POOL_SIZE") {
            Ok(variable) => {
                match variable.parse::<u32>() {
                    Ok(size) => size,
                    Err(_) => {
                        warn!(target: "config", "The DATABASE_POOL_SIZE variable must be an unsigned integer.");

                        DefaultConfig::DEFAULT_DATABASE_POOL_SIZE
                    },
                }
            },
            Err(_) => DefaultConfig::DEFAULT_DATABASE_POOL_SIZE,
        };

        Ok(DefaultConfig {
            database_url,
            entry_point,
            database_pool_size,
            with_database,
        })
    }
}

/*   -------------------------------------------------------------
     MinimalConfig

     :: Config
     :: sui generis implementation
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// This is a minimal implementation of the `Config` trait, which extracts the following variables
/// from an .env file or environment:
///
///   - `API_ENTRY_POINT` (facultative, by default `/`): the mounting point of the API methods
///
///  It sets the server not to use a database.
pub struct MinimalConfig {
    entry_point: String,
}

impl Config for MinimalConfig {
    fn get_database_url(&self) -> &str {
        ""
    }

    fn get_entry_point(&self) -> &str {
        &self.entry_point
    }

    fn get_database_pool_size(&self) -> u32 {
        0
    }

    fn with_database(&self) -> bool { false }

    fn into_service(self, routes: Vec<Route>) -> Box<dyn Service> {
        let service = MinimalService {
            config: self,
            routes,
        };

        Box::new(service)
    }
}

impl EnvironmentConfigurable for MinimalConfig {
    fn parse_environment() -> ErrorResult<Self> {
        if let Err(error) = dotenv() {
            warn!(target: "config", "Can't parse .env: {}", error);
        };

        let entry_point = env::var("API_ENTRY_POINT").unwrap_or(String::from("/"));

        Ok(MinimalConfig {
            entry_point,
        })
    }
}
