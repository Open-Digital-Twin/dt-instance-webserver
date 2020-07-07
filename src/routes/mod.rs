pub mod user;
pub mod element;
pub mod source;
pub mod twin;

use crate::common::models::response::{Response};
use actix_web::{HttpResponse};

pub fn handle_req_error(error: String, status: usize) -> HttpResponse {
  let mut response = match status {
    400 => HttpResponse::BadRequest(),
    404 => HttpResponse::NotFound(),
    500 => HttpResponse::InternalServerError(),
    _ => HttpResponse::BadRequest()
  };

  response.json(Response {
    message: error,
    status: false
  })
}
