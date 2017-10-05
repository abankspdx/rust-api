/// Types for working with IronResult<Response>
/// I found there was kind of a lot of boilerplate in each handler
/// So I wrote these types/methods to make life a little easier

use iron::prelude::*;
use iron::mime::Mime;

use iron::status::Status;
use serde::ser::Serialize;

pub struct ResultWithBody<T: Serialize> {
    pub status: Status,
    pub body: T
}

impl<T: Serialize> ResultWithBody<T> {
    pub fn return_result(&self) -> IronResult<Response> {
        let content_type = "application/json".parse::<Mime>().unwrap();
        let raw = &self.body;
        let payload = json!(raw).to_string();
        return Ok(Response::with((content_type, self.status, payload)))
    }
}

pub struct ResultWithoutBody {
    pub status: Status,
    pub message: Option<String>
}

impl ResultWithoutBody {
    pub fn return_result(&self) -> IronResult<Response> {
        let content_type = "application/json".parse::<Mime>().unwrap();
        match self.message.clone() {
            Some(msg) => return Ok(Response::with((content_type, self.status, json!(msg).to_string()))),
            None => return Ok(Response::with((content_type, self.status)))
        }
    }
}


