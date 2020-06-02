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
#[put("/")]
async fn put_element(
  _auth: AuthValidator,
  _env: web::Data<Environment>,
  session: web::Data<Arc<CurrentSession>>,
  element: web::Json<ElementRegister>
) -> HttpResponse {
  let _element = Element {
    id: uuid::Uuid::new_v4().to_string(),
    twin: _env.twin_instance.to_string(),
    name: element.name.to_string(),
    parent: element.parent.to_string(), // optional parent element
    created_at: "toTimestamp(now())"
  };

  match insert_element(session, _element) {
    Ok(element) => {
      HttpResponse::Ok().json(Response {
        message: format!("Success in creating element {}.", element.name.to_string()),
        status: true
      })
    },
    Err(_) => {
      HttpResponse::Ok().json(Response {
        message: format!("Error in creating element."),
        status: false
      })
    }
  }
}

fn get_element_by_id(session: web::Data<Arc<CurrentSession>>, id: String, twin: String) -> Result<User, String> {
  let rows = session.query_with_values(
    "SELECT * FROM element WHERE id = ? AND twin = ? ALLOW FILTERING",
    query_values!("id" => id, "twin" => twin)
  )
  .expect("select by id the element of twin")
    .get_body().unwrap()
    .into_rows().unwrap();

  if !rows.is_empty() {
    let element = match Element::try_from_row(rows[0].clone()) {
      Ok(_model) => _model,
      Err(_) => return Err("Could not convert rows to Element model.".to_string())
    };
    return Ok(element);
  }
  return Err("No element with informed id on this twin".to_string());
}

fn insert_element(session: web::Data<Arc<CurrentSession>>, element: Element) -> Result<Element, String> {
  let r = session.query_with_values(
    "INSERT INTO element (id, twin, name, created_at, parent) VALUES (?, ?, ?, ?, ?)",
    element.to_query()
  ).expect("Inserted new element");

  info!("New element {}:{} of twin {}.", element.id, element.name, element.twin);

  match r {
    Ok(_) => Ok(element),
    Err(_) => Err("Error inserting element.")
  }
}

fn delete_element_by_id(session: web::Data<Arc<CurrentSession>>, id: String, twin: String) -> Result<String, String> {
  let r = session.query_with_values(
    "DELETE FROM element WHERE id = ? AND twin = ?",
    query_values!("id" => id, "twin" => twin)
  )
    .expect("Delete by id the element of twin");
  
  match r {
    Ok(_) => Ok(format!("Success deleting element {}.", id)),
    Err(_e) => Err(format!("Error deleting element.")),
  }  
}

// create_source
// update_source
// delete_source
// clear_source_data (Delete all from timestamp interval)

// insert_data

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(put_element);
  // cfg.service(login);
  // cfg.service(register);
}
