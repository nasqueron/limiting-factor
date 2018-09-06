extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate log;
extern crate r2d2;
extern crate rocket;
extern crate rocket_contrib;

/*   -------------------------------------------------------------
     Public modules offered by this crate
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub mod api;
pub mod config;
pub mod database;
pub mod kernel;

/*   -------------------------------------------------------------
     Custom types
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub type ErrorResult<T> =  Result<T, Box<dyn std::error::Error>>;
