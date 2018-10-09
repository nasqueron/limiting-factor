//! A library with components to implement a REST API.
//!
//! The goal of this crate is to provide:
//!
//!   - boilerplate to parse environment and run a Rocket server
//!   - glue code for Rocket and Diesel to use a database in the web service
//!   - standard API replies
//!
//!  That allows an API or a back-end web server to focus on requests and data model.
//!
//!  # Examples
//!
//!  A simple server serving a 200 ALIVE response on /status :
//!
//!  ```no_run
//!  use limiting_factor::kernel::DefaultApplication;
//!
//!  pub fn run () {
//!      let routes = routes![
//!          status,
//!      ];
//!
//!      DefaultApplication::start_application(routes);
//!  }
//!
//!  #[get("/status")]
//!  pub fn status() -> &'static str {
//!      "ALIVE"
//!  }
//!  ```
//!
//! Replacing `DefaultApplication` by `MinimalApplication` allows to use a lighter version
//! of the library without Diesel dependencies or database use.

#[cfg(feature = "pgsql")]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
#[cfg(feature = "pgsql")]
extern crate r2d2;
extern crate rocket;
extern crate rocket_contrib;

/*   -------------------------------------------------------------
     Public modules offered by this crate
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub mod api;
pub mod config;
pub mod kernel;

/*   -------------------------------------------------------------
     Optional public features modules offered by this crate
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

#[cfg(feature = "pgsql")]
pub mod database;

/*   -------------------------------------------------------------
     Custom types
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub type ErrorResult<T> =  Result<T, Box<dyn std::error::Error>>;
