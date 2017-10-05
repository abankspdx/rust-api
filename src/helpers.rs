use iron::prelude::*;

use iron::status::Status;
use serde::ser::Serialize;

use types::handler_result::{ResultWithBody, ResultWithoutBody};

/// A convenience function to be used by Handlers
/// The goal is to not have to manually verify `Option<T>` service results
/// Simply provide the `Option<T>`, a success/failure status, and an error message
/// The message is only used in the event of an error
pub fn build_response<T: Serialize>(thing: Option<T>,
                                    status_true: Status,
                                    status_false: Status,
                                    error_message: Option<String>)
                                    -> IronResult<Response> {
    if thing.is_some() {
        let raw = thing.unwrap();
        let result: ResultWithBody<T> = ResultWithBody {
            body: raw,
            status: status_true
        };
        return result.return_result()
    } else {
        let result = ResultWithoutBody {
            status: status_false,
            message: error_message
        };
        return result.return_result()
    }
}

/// If there isn't a `Option<T>`, but just a status and string
/// This is used more frequently for errors, less for GET bodies
pub fn respond_empty(status: Status, message: Option<String>) -> IronResult<Response> {
    let result = ResultWithoutBody {
        status: status,
        message: message
    };
    return result.return_result()
}