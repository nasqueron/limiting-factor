The **limiting-factor** crate offers facilities to implement a REST API.

## Goal

The goal of this library is to provide:

  - glue code for Rocket and Diesel
  - standard API replies
  - boilerplate to parse environment to extract configuration and run a server

That allows an API or a back-end web server to focus on requests and data model,
and to maintain helper methods as a separate library.

## Dependencies

* Diesel, as PostgreSQL ORM, with r2d2 support to pool connections
* Rocket, as web framework
* Chrono, for date and time types 

## Credits

The glue code to use Rocket with Diesel is adapted from the Rocket guide.
See https://rocket.rs/guide/state/#databases. Guide author: Sergio Benitez.
