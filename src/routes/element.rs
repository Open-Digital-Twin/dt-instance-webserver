use cdrs::query::*;
use cdrs::frame::traits::TryFromRow;

use crate::common::models::app::{CurrentSession, Environment};
use crate::common::models::response::{Response, DataResponse};
use crate::common::models::twin::*;
use crate::common::db::get_by_id;

use crate::middlewares::auth::AuthValidator;

use std::sync::Arc;

use log::{info};
use actix_web::{get, put, web, HttpResponse};

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

// fn get_element_by_id(session: web::Data<Arc<CurrentSession>>, id: String, twin: String) -> Result<User, String> {
//   let rows = session.query_with_values(
//     "SELECT * FROM element WHERE id = ? AND twin = ? ALLOW FILTERING",
//     query_values!("id" => id, "twin" => twin)
//   )
//   .expect("select by id the element of twin")
//     .get_body().unwrap()
//     .into_rows().unwrap();

//   if !rows.is_empty() {
//     let element = match Element::try_from_row(rows[0].clone()) {
//       Ok(_model) => _model,
//       Err(_) => return Err("Could not convert rows to Element model.".to_string())
//     };
//     return Ok(element);
//   }
//   return Err("No element with informed id on this twin".to_string());
// }

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
    Err((error, status)) => {
      let mut response;

      match status {
        400 => response = HttpResponse::BadRequest(),
        404 => response = HttpResponse::NotFound(),
        _ => response = HttpResponse::BadRequest()
      }

      response.json(Response {
        message: error,
        status: false
      })
    }
  }
}

// fn delete_element_by_id(session: web::Data<Arc<CurrentSession>>, id: String, twin: String) -> Result<String, String> {
//   let r = session.query_with_values(
//     "DELETE FROM element WHERE id = ? AND twin = ?",
//     query_values!("id" => id, "twin" => twin)
//   )
//     .expect("Delete by id the element of twin");
  
//   match r {
//     Ok(_) => Ok(format!("Success deleting element {}.", id)),
//     Err(_e) => Err(format!("Error deleting element.")),
//   }  
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(put_element);
  cfg.service(get_element);
}
