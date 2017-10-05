# Rust API Example
###Written with Iron, Diesel, Docker

This is not meant to be "production" ready by any means. I just was having trouble finding a "full" example of an API in Rust, so I decided to make one to better understand. I've been working with Rust for only a few weeks, so this was mostly for me to learn. I thought others might find use for it too.

###Structure
There are three "layers" to this API.

- Handlers
- Services
- Types

This is generally how I construct APIs in other languages as well. Handlers coalesce HTTP requests into business logic units, which live in Services. Services interract with base datatypes (Types). Types also contain required traits for Diesel to run.

I've included a basic JWT implementation and the surrounding Iron code, with a couple examples of where/how you'd use it.

###Tests
I wrote tests for JWT and Password hashing services, as those services aren't bound to an HTTP request/Database connection. You could probably make this a bit more "testable," whether by implementing DI or constructing struct-based service instances.

###Libraries used
- iron = "0.5.1"
- router = "0.5.1"
- time = "0.1.38"
- serde = "1.0.11"
- serde_derive = "1.0.11"
- serde_json = "1.0.2"
- diesel = { version = "0.16.0", features = ["postgres"] }
- diesel_codegen = { version = "0.16.0", features = ["postgres"] }
- dotenv = "0.9.0"
- bcrypt = "0.1"
- rand = "0.3"
- bodyparser = "0.7.0"
- frank_jwt = "2.5.1"

###Code reviews are welcome!