use cdrs::query_values;
use cdrs::query::*;
use cdrs::frame::TryFromRow;

// use crate::middlewares::auth::AuthorizationService;
use crate::models::user::{User, UserLogin, Claims, Register};
use crate::models::app::{Environment};
use crate::models::response::{LoginResponse, Response};
use crate::models::group::{Group};
use crate::{CurrentSession};
use crate::middlewares::auth::AuthValidator;
use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use argon2::{self, Config};
use rand::{ thread_rng, Rng };
use rand::distributions::Alphanumeric;

// use crate::routes::user::{IUserRepository, UserRepository};
// use actix_web::http::StatusCode;
// use actix_web::{post, get, web, HttpRequest, HttpResponse};
use actix_web::{get, post, put, web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};

/// Create an element in the twin instance.
/// Element is a general definition for a collection of "things" that define a Twin.
/// Elements can include other elements, like devices.
/// Elements have the ability to define multiple sources of data (device sensors, entry points of data).
#[put("/element")]
async fn create_element(
  _auth: AuthValidator,
  _env: web::Data<Environment>,
  session: web::Data<Arc<CurrentSession>>,
  element: web::Json<ElementRegister>
) -> HttpResponse {
  let _element = get_element_by_name(session.clone(), element.name.clone(), _env.twin_instance);

  // if no element, create element.
  // else, return error.
}

// get_element_by_id
// update_element_by_id
// delete_element_by_id

// create_source
// update_source
// delete_source
// clear_source_data (Delete all from timestamp interval)

// insert_data

fn get_element_by_name(session: web::Data<Arc<CurrentSession>>, name: String, twin: i64) -> Result<User, String> {
  let rows = session.query_with_values(
    "SELECT * FROM element WHERE name = ? AND twin = ? ALLOW FILTERING",
    query_values!("name" => name, "twin" => twin)
  )
    .expect("select by name the element of twin")
    .get_body().unwrap()
    .into_rows().unwrap();

  if !rows.is_empty() {
    let element = match Element::try_from_row(rows[0].clone()) {
      Ok(_model) => _model,
      Err(_) => return Err("Could not convert rows to Element model.".to_string())
    };

    return Ok(element);
  }
  return Err("No element with selected name on this twin".to_string());
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(create_element);
  // cfg.service(login);
  // cfg.service(register);
}
