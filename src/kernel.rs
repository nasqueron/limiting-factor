//! # Service execution utilities.
//!
//! Provides methods to start the server and handle the application

use config::{Config, MinimalConfig};
#[cfg(feature = "pgsql")]
use config::DefaultConfig;
#[cfg(feature = "pgsql")]
use database::{initialize_database_pool, test_database_connection};
use ErrorResult;
use rocket::Route;
use rocket::ignite;
use std::process;
use std::marker::PhantomData;
use config::EnvironmentConfigurable;

/*   -------------------------------------------------------------
     Service

     Allow to define config and routes. Launch a server.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub trait Service {
    fn get_config(&self) -> &dyn Config;

    fn get_routes(&self) -> &[Route];

    fn launch_server(&mut self) -> ErrorResult<()>;

    fn check_service_configuration(&self) -> ErrorResult<()>;

    fn run (&mut self) -> ErrorResult<()> {
        info!(target: "runner", "Server started.");

        {
            self.check_service_configuration()?
        }

        self.launch_server()?;

        Ok(())
    }
}

/*   -------------------------------------------------------------
     Default service

     Allow to define config and routes. Launch a server.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// The default service offers a pgsql database connection with Diesel and r2d2.
#[cfg(feature = "pgsql")]
pub struct DefaultService {
    pub config: DefaultConfig,
    pub routes: Vec<Route>,
}

#[cfg(feature = "pgsql")]
impl Service for DefaultService {
    fn get_config(&self) -> &dyn Config { &self.config }

    fn get_routes(&self) -> &[Route] { self.routes.as_slice() }

    fn launch_server(&mut self) -> ErrorResult<()> {
        let config = self.get_config();
        let routes = self.get_routes();

        let mut server = ignite();

        if config.with_database() {
            server = server.manage(
                initialize_database_pool(config.get_database_url(), config.get_database_pool_size())?
            );
        }

        server
            .mount(config.get_entry_point(), routes.to_vec())
            .launch();

        Ok(())
    }

    fn check_service_configuration(&self) -> ErrorResult<()> {
        let config = self.get_config();
        if config.with_database() {
            test_database_connection(config.get_database_url())?;
            info!(target: "runner", "Connection to database established.");
        }

        Ok(())
    }
}

/*   -------------------------------------------------------------
     Minimal service

     Allow to define config and routes. Launch a server.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// The minimal service allows to spawn a server without any extra feature.
pub struct MinimalService {
    pub config: MinimalConfig,
    pub routes: Vec<Route>,
}

impl Service for MinimalService {
    fn get_config(&self) -> &dyn Config { &self.config }

    fn get_routes(&self) -> &[Route] { self.routes.as_slice() }

    fn launch_server(&mut self) -> ErrorResult<()> {
        let config = self.get_config();
        let routes = self.get_routes();

        ignite()
            .mount(config.get_entry_point(), routes.to_vec())
            .launch();

        Ok(())
    }

    fn check_service_configuration(&self) -> ErrorResult<()> { Ok(()) }
}

/*   -------------------------------------------------------------
     Base application as concrete implementation

     :: Application
     :: sui generis implementation
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// The application structure allows to encapsulate the service into a CLI application.
///
/// The application takes care to run the service and quits with a correct exit code.
///
/// It also takes care of initialisation logic like parse the environment to extract
/// the configuration.
pub struct Application<U>
    where U: Config
{
    service: Box<dyn Service>,
    config_type: PhantomData<U>,
}

impl<U> Application<U>
    where U: Config + EnvironmentConfigurable
{
    pub fn new (config: U, routes: Vec<Route>) -> Self {
        Application {
            service: config.into_service(routes),
            config_type: PhantomData,
        }
    }

    /// Starts the application
    ///
    /// # Exit codes
    ///
    /// The software will exit with the following error codes:
    ///
    ///   - 0: Graceful exit (currently not in use, as the application never stops)
    ///   - 1: Error during the application run (e.g. routes conflict or Rocket fairings issues)
    ///   - 2: Error parsing the configuration (e.g. no database URL has been defined)
    pub fn start (&mut self) {
        info!(target: "runner", "Server initialized.");

        if let Err(error) = self.service.run() {
            error!(target: "runner", "{}", error);
            process::exit(1);
        }

        process::exit(0);
    }

    pub fn start_application (routes: Vec<Route>) {
        let config = <U>::parse_environment().unwrap_or_else(|_error| {
            process::exit(2);
        });

        let mut app = Application::new(config, routes);
        app.start();
    }
}

/*   -------------------------------------------------------------
     Default application

     :: Application
     :: sui generis implementation, wrapper for Application
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
#[cfg(feature = "pgsql")]
pub struct DefaultApplication {}

#[cfg(feature = "pgsql")]
impl DefaultApplication {
    pub fn start_application (routes: Vec<Route>) {
        Application::<DefaultConfig>::start_application(routes);
    }
}

/*   -------------------------------------------------------------
     Minimal application

     :: Application
     :: sui generis implementation, wrapper for Application
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub struct MinimalApplication {}

impl MinimalApplication {
    pub fn start_application (routes: Vec<Route>) {
        Application::<MinimalConfig>::start_application(routes);
    }
}
