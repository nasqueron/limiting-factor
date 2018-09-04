extern crate diesel;
#[macro_use] extern crate log;
extern crate r2d2;
extern crate rocket;
extern crate rocket_contrib;

/*   -------------------------------------------------------------
     Public modules offered by this crate
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub mod api;
pub mod database;

/*   -------------------------------------------------------------
     Custom types
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub type ErrorResult<T> =  Result<T, Box<dyn std::error::Error>>;
