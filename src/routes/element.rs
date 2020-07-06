use cdrs::query::*;
use cdrs::frame::traits::TryFromRow;

use crate::common::models::app::{CurrentSession, Environment};
use crate::common::models::response::{Response, DataResponse, VecDataResponse};
use crate::common::models::twin::*;
use crate::db::{get_by_id, get_element_sources, delete_by_id, delete_where_in};

use crate::middlewares::auth::AuthValidator;
use crate::routes::handle_req_error;

use std::sync::Arc;

use log::{info};
use actix_web::{delete, get, put, web, HttpResponse};

/// Create an element in the twin instance.
/// Element is a general definition for a collection of "things" that define a Twin.
/// Elements can include other elements, like devices.
/// Elements have the ability to define multiple sources of data (device sensors, entry points of data).
#[put("")]
async fn put_element(
  _auth: AuthValidator,
  _env: web::Data<Environment>,
  session: web::Data<Arc<CurrentSession>>,
  register: web::Json<ElementRegister>
) -> HttpResponse {
  let _element = Element {
    id: uuid::Uuid::new_v4(),
    twin: _env.twin_instance,
    name: register.name.to_string(),
    // created_at: chrono::offset::Utc::now(),
    parent: register.parent
  };

  match insert_element(session, &_element) {
    Ok(response) => {
      HttpResponse::Created().json(DataResponse {
        message: format!("{}", response),
        status: true,
        data: _element
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

fn insert_element(session: web::Data<Arc<CurrentSession>>, element: &Element) -> Result<String, String> {
  let r;

  if element.parent == None {
    r = session.query_with_values(
      "INSERT INTO element (id, twin, name, created_at) VALUES (?, ?, ?, toTimestamp(now()))",
      element.clone().to_query_no_parent()
    );
  } else {
    r = session.query_with_values(
      "INSERT INTO element (id, twin, name, parent, created_at) VALUES (?, ?, ?, ?, toTimestamp(now()))",
      element.clone().to_query()
    );
  }

  match r.expect("Inserted new element").get_body() {
    Ok(_) => {
      let resp = format!("New element {}:{} of twin {}.", element.id, element.name, element.twin);
      info!("{}", resp);
      Ok(resp)
    },
    Err(_) => Err(format!("Error inserting element."))
  }
}

#[get("{element_id}")]
async fn get_element(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  element_id: web::Path<String>
) -> HttpResponse {
  match get_by_id::<Element>(session, element_id.to_string(), "element".to_string()) {
    Ok(element) => HttpResponse::Ok().json(DataResponse {
      message: format!("Found element {}", element.clone().id),
      data: element,
      status: true
    }),
    Err((error, status)) => handle_req_error(error, status)
  }
}

#[get("{element_id}/sources")]
async fn get_sources_by_element(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  element_id: web::Path<String>
) -> HttpResponse {
  match get_element_sources(session, element_id.to_string()) {
    Ok(sources) => HttpResponse::Ok().json(VecDataResponse {
      message: format!("Found {} sources for element {}", sources.len(), element_id),
      data: sources,
      status: true
    }),
    Err((error, status)) => handle_req_error(error, status)
  }
}

#[delete("{element_id}")]
async fn delete_element(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  element_id: web::Path<String>
) -> HttpResponse {
  let mut message = String::new();
  let id = element_id.to_string();

  // Delete element
  match delete_by_id(session.clone(), id.clone(), "element".to_string()) {
    Ok(delete_element_message) => {
      message.push_str(delete_element_message.as_str());
      message.push('\n');

      // Get sources of element
      let sources: Vec<String> = match get_element_sources(session.clone(), element_id.to_string()) {
        Ok(_sources) => _sources.into_iter().map(|s| s.id.to_string()).collect(),
        Err((error, status)) => return handle_req_error(error, status)
      };

      // Delete element sources
      match delete_where_in(session.clone(), sources.clone(), "source".to_string(), "id".to_string()) {
        Ok(delete_source_message) => {
          message.push_str(delete_source_message.as_str());
          message.push('\n');

          // Delete element sources data
          match delete_where_in(session, sources, "source_data".to_string(), "source".to_string()) {
            Ok(delete_source_data_message) => {
              message.push_str(delete_source_data_message.as_str());
              message.push('\n');

              HttpResponse::Ok().json(Response {
                message,
                status: true
              })
            },
            Err((error, status)) => handle_req_error(error, status)
          }
        },
        Err((error, status)) => handle_req_error(error, status)
      }
    },
    Err((error, status)) => handle_req_error(error, status)
  }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(put_element);
  cfg.service(get_element);
  cfg.service(get_sources_by_element);
  cfg.service(delete_element);
}
