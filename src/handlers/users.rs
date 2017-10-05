/// Without commenting on each route handler, each handler takes a request
/// and returns an IronResult<Response>
/// I wrote convenience functions/types around those results, so each handler
/// could be more concise.
/// If these were behind JWT routes, a handler could pull the stringified user out
/// of the request.

extern crate iron;
extern crate router;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use router::Router;

use services;
use types::user::{UserRegister, UserAuthenticate};
use helpers::{build_response, respond_empty};

pub fn get_users(_: &mut Request) -> IronResult<Response> {
    let users = services::users::get_users();
    return build_response(users, status::Ok, status::NoContent, None)
}

pub fn get_user(req: &mut Request) -> IronResult<Response> {
    let ref id_ref = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("0");
    let id: i32 = id_ref.parse().expect("Wanted a number");
    let user = services::users::get_user(id);

    return build_response(user, status::Ok, status::NotFound, Some("User not found".to_string()))
}

pub fn authenticate(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<UserAuthenticate>>();

    match body {
        Ok(Some(body)) => return authenticate_user(body),
        Ok(None) => return respond_empty(status::NotFound, Some("User not found".to_string())),
        Err(_) => return respond_empty(status::BadRequest, None)
    }
}

fn authenticate_user(user: UserAuthenticate) -> IronResult<Response> {
    let correct = services::users::authenticate(user);
    match correct {
        Some(x) => respond_empty(status::Ok, Some(x)),
        None => respond_empty(status::BadRequest, None)
    }
}

pub fn register(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<UserRegister>>();
    match body {
        Ok(Some(body)) => return register_user(body),
        Ok(None) => return respond_empty(status::BadRequest, None),
        Err(err) => return respond_empty(status::BadRequest, Some(err.to_string())),
    }
}

// I'm not sure why IntelliJ complains about the match statement below
// It compiles and works correctly, but the IDE still complains about return types
fn register_user(user: UserRegister) -> IronResult<Response> {
    let new_user = services::users::create_user(user);
    match new_user {
        Ok(created) => return build_response(Some(created), status::Ok, status::BadRequest, None),
        Err(e) => return respond_empty(status::BadRequest, Some(e.to_string()))
    };
}


