// use cdrs::query_values;
use cdrs::query::*;
// use cdrs::frame::TryFromRow;

// use crate::middlewares::auth::AuthorizationService;
// use crate::models::user::*;
use crate::models::app::{Environment};
use crate::models::response::{Response, DataResponse};
use crate::models::twin::*;

use crate::{CurrentSession};
use crate::middlewares::auth::AuthValidator;
use std::sync::Arc;

use log::{info};
// use chrono::{DateTime, Duration, Utc};
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
// use argon2::{self, Config};
// use rand::{ thread_rng, Rng };
// use rand::distributions::Alphanumeric;

// use crate::routes::user::{IUserRepository, UserRepository};
// use actix_web::http::StatusCode;
// use actix_web::{post, get, web, HttpRequest, HttpResponse};
use actix_web::{put, web, HttpResponse};
// use jsonwebtoken::{encode, EncodingKey, Header};

/// Create an element in the twin instance.
/// Element is a general definition for a collection of "things" that define a Twin.
/// Elements can include other elements, like devices.
/// Elements have the ability to define multiple sources of data (device sensors, entry points of data).
#[put("")]
async fn put_source(
  _auth: AuthValidator,
  _env: web::Data<Environment>,
  session: web::Data<Arc<CurrentSession>>,
  register: web::Json<SourceRegister>
) -> HttpResponse {
  let _source = Source {
    id: uuid::Uuid::new_v4(),
    name: register.name.to_string(),
    element: register.element,
    created_at: chrono::offset::Utc::now()
  };

  match insert_source(session, &_source) {
    Ok(source) => {
      let answer = format!("Created source {}:{} of element {}.", source.id, source.name, source.element);
      info!("{}", answer);

      HttpResponse::Ok().json(DataResponse {
        message: answer,
        status: true,
        data: source
      })
    },
    Err(_) => {
      HttpResponse::Ok().json(Response {
        message: format!("Error in creating source."),
        status: false
      })
    }
  }
}

fn insert_source(session: web::Data<Arc<CurrentSession>>, source: &Source) -> Result<Source, String> {
  let r = session.query_with_values(
    "INSERT INTO source (id, name, element, created_at) VALUES (?, ?, ?, toTimestamp(now()))",
    source.clone().to_query()
  );

  match r.expect("Inserted new data source").get_body() {
    Ok(_) => Ok(source.clone()),
    Err(_) => Err(format!("Error inserting data source."))
  }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(put_source);
}
