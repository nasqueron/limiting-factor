//! # API module
//!
//! Provides methods to start the server and handle the application

use config::Config;
use config::DefaultConfig;
use database::initialize_database_pool;
use database::test_database_connection;
use ErrorResult;
use rocket::Route;
use rocket::ignite;
use std::process;

/*   -------------------------------------------------------------
     Application

     Allow to define config and routes. Launch a server.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub trait Application {
    fn get_config(&self) -> &dyn Config;

    fn get_routes(&self) -> &[Route];

    fn launch_server(&mut self) -> ErrorResult<()> {
        let config = self.get_config();
        let routes = self.get_routes();

        ignite()
            .manage(
                initialize_database_pool(config.get_database_url(), config.get_database_pool_size())?
            )
            .mount(config.get_entry_point(), routes.to_vec())
            .launch();

        Ok(())
    }

    fn run (&mut self) -> ErrorResult<()> {
        info!(target: "runner", "Server started.");

        // Initial connection to test if the database configuration works
        {
            let config = self.get_config();
            test_database_connection(config.get_database_url())?;
            info!(target: "runner", "Connection to database established.");
        }

        self.launch_server()?;

        Ok(())
    }
}

/*   -------------------------------------------------------------
     Default application

     :: Application
     :: sui generis implementation
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// The default application implements CLI program behavior to prepare a configuration from the
/// `DefaultConfig` implementation, test if it's possible to connect to the database, and if so,
/// launch a Rocket server.
///
/// # Examples
///
/// To run an application with some routes in a `requests` module:
///
/// ```
/// use limiting_factor::kernel::DefaultApplication;
/// use requests::*;
///
/// pub fn main () {
///     let routes = routes![
///         status,
///         favicon,
///         users::register,
///         users::get_player,
///     ];
///
///     DefaultApplication::start_application(routes);
/// }
/// ```
///
/// The default configuration will be used and the server started.
pub struct DefaultApplication {
    config: DefaultConfig,
    routes: Box<Vec<Route>>,
}

impl Application for DefaultApplication {
    fn get_config(&self) -> &dyn Config {
        &self.config
    }

    fn get_routes(&self) -> &[Route] {
        self.routes.as_slice()
    }
}

impl DefaultApplication {
    pub fn new (config: DefaultConfig, routes: Vec<Route>) -> Self {
        DefaultApplication {
            config,
            routes: Box::new(routes),
        }
    }

    /// Starts the application, prepares default configuration
    ///
    /// # Exit codes
    ///
    /// The software will exit with the following error codes:
    /// 0: Exits gracefully (but currently we don't have a signal to ask the server to shutdown)
    /// 1: Error during the application run (e.g. routes conflict or Rocket fairings issues)
    /// 2: Error parsing the configuration (e.g. no database URL has been defined)
    pub fn start_application (routes: Vec<Route>) {
        info!(target: "runner", "Server initialized.");

        let config = DefaultConfig::parse_environment().unwrap_or_else(|_error| {
            process::exit(2);
        });

        let mut app = Self::new(config, routes);

        if let Err(error) = app.run() {
            error!(target: "runner", "{}", error.description());
            process::exit(1);
        }

        process::exit(0);
    }
}
