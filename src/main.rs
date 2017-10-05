extern crate iron;
extern crate time;
extern crate router;
extern crate dotenv;
extern crate rand;
extern crate bcrypt;
extern crate bodyparser;
extern crate serde;
extern crate frank_jwt;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use router::Router;
use iron::{BeforeMiddleware, typemap};
use std::error::Error;
use iron::status;
use std::fmt::{self, Debug};


mod handlers;
mod services;
mod types;
mod database;
pub mod schema;
mod helpers;

/// A test route, just to make sure the API is running
fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

/// StringError is just a convenience struct to
/// work with IronResult/IronError
#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

impl typemap::Key for types::user::User { type Value = String; }

/// The Iron-specific code for verifying a JWT token before entering a route
/// This adds the principal to the request, to be used in any following route handler
struct AuthorizationMiddleware;
impl BeforeMiddleware for AuthorizationMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let raw_header = req.headers.get_raw("authorization");
        match raw_header {
            Some(header) => {
                let val = header[0].clone();
                let user = ::services::jwt::validate_header(val);
                match user {
                    Some(u) => {
                        req.extensions.insert::<types::user::User>(json!(u).to_string());
                        Ok(())
                    },
                    None => return_error()
                }
            },
            None => return_error()
        }
    }
}

/// A convenience function to return an IronResult error
/// There are various ways this can be invoked during the JWT process
/// I just wanted to standardize that code
fn return_error() -> IronResult<()> {
    return Err(IronError::new(StringError("Error".to_string()), status::BadRequest))
}

/// Register all the routes, creating chains for JWT if necessary
/// If this moved foward, moving the routes to their own function would be wise
/// Also, adding a logging chain link would probably be wise
fn main() {
    let mut router = Router::new();
    // public
    router.get("/", hello_world, "index");
    router.post("/user", handlers::users::register, "register");
    router.post("/user/authenticate", handlers::users::authenticate, "authenticate");


    // authenticated
    let mut user_chain = Chain::new(handlers::users::get_user);
    user_chain.link_before(AuthorizationMiddleware);
    router.get("/user/:id", user_chain, "user");

    let mut users_chain = Chain::new(handlers::users::get_users);
    users_chain.link_before(AuthorizationMiddleware);
    router.get("/users", users_chain, "users");


    println!("Listening on port 3000");
    Iron::new(router).http("0.0.0.0:3000").unwrap();
}



