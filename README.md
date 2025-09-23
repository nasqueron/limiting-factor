The **limiting-factor** crate offers facilities to implement a REST API.

## Goal

The goal of this library is to provide:

  - standard API replies
  - boilerplate to parse environment to extract configuration and run a server
  - glue code for database support

That allows an API or a back-end web server to focus on requests and data model,
and to maintain helper methods as a separate library.

## Dependencies

The library core features rely on:
  * Chrono for date and time types
  * Serde for serialization

If you need PostgreSQL support:
  * Diesel as ROM
  * r2d2 to pool connections

## Pick your crate

Currently, we provide and support only `limiting-factor` crate for Rocket 0.4.

We're also working on a new crate to ease migration from Rocket to Axum.

The exact crate to use depends on your framework:

| Framework      | Crate name                      | To build with            |
|----------------|---------------------------------|--------------------------|
| Axum           | limiting-factor-axum            | stable or nightly        |
| Rocket 0.3     | limiting-factor v0.5.1          | (1)                      |
| Rocket 0.4     | limiting-factor v0.8.0          | nightly-2024-12-15       |
| Rocket 0.5     | -                               | *not yet supported.*     |

(1) Build of Rocket 0.3 seems to be tricky and require custom dependencies:
rocket v0.3.16 or v0.3.17 -> cookie v0.9.1 -> ring v0.11.0, a yanked crate

## Compile

### Nightly for Rocket builds

If you want to target Rocket before 0.5, you need a nightly toolchain.

Stable toolchain will work for other crates.

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
### Rocket x Diesel glue code

The glue code to use Rocket with Diesel is adapted from Rocket v0.3 guide.
See https://rocket.rs/guide/v0.3/state/#databases.

Guide author: Sergio Benitez.
