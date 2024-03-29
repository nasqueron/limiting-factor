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

## Compile

### Windows

You need to give to `rustc` some hints about where `libpq.lib` is.

The pq-sys crate offers a build script to find the library
and then print the relevant hints.

You can manually set your PostgreSQL library folder with:

```
export PQ_LIB_DIR="C:\Program Files\PostgreSQL\13\lib"
cargo run
```

## Credits

The glue code to use Rocket with Diesel is adapted from the Rocket guide.
See https://rocket.rs/guide/state/#databases. Guide author: Sergio Benitez.
