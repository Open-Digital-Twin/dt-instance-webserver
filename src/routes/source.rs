use cdrs::query::*;
use cdrs::query_values;
use cdrs::frame::traits::TryFromRow;

use crate::common::models::app::{Environment, SOURCE_DATA_TOPIC, SOURCE_DATA_ACK_TOPIC};
use crate::common::models::response::{Response, DataResponse, DataResponseWithTopics};
use crate::common::models::twin::*;
use crate::common::models::request::{DataInterval};

use crate::db::{get_by_id, delete_by_id, delete_by_where};

use crate::{CurrentSession};
use crate::middlewares::auth::AuthValidator;
use crate::routes::handle_req_error;

use std::sync::Arc;

use log::{info};
use actix_web::{get, put, delete, web, HttpResponse};

use std::collections::HashMap;

use uuid::Uuid;

/// Create a data source in an element of the twin instance.
#[put("")]
async fn put_source(
  _auth: AuthValidator,
  _env: web::Data<Environment>,
  session: web::Data<Arc<CurrentSession>>,
  register: web::Json<SourceRegister>
) -> HttpResponse {
  let _source = Source {
    id: Uuid::new_v4(),
    name: register.name.to_string(),
    element: register.element,
    // created_at: chrono::offset::Utc::now()
  };

  match insert_source(session, &_source) {
    Ok(source) => {
      let answer = format!("Created source {}:{} of element {}.", source.id, source.name, source.element);
      info!("{}", answer);

      let mut topics: HashMap<String, String> = HashMap::new();
      topics.insert(SOURCE_DATA_TOPIC.to_string(), source.clone().data_topic());
      topics.insert(SOURCE_DATA_ACK_TOPIC.to_string(), source.clone().data_ack_topic());

      HttpResponse::Created().json(DataResponseWithTopics {
        topics,
        message: answer,
        status: true,
        data: source
      })
    },
    Err(_) => {
      HttpResponse::InternalServerError().json(Response {
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

#[get("{source_id}")]
async fn get_source(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  source_id: web::Path<String>
) -> HttpResponse {
  match get_by_id::<Source>(session, source_id.to_string(), "source".to_string()) {
    Ok(source) => HttpResponse::Ok().json(DataResponse {
      message: format!("Found source {}", source.clone().id),
      data: source,
      status: true
    }),
    Err((error, status)) => handle_req_error(error, status)
  }
}

#[delete("{source_id}")]
async fn delete_source(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  source_id: web::Path<String>
) -> HttpResponse {
  let id: String = source_id.to_string();

  // Delete source.
  match delete_by_id(session.clone(), id.clone(), "source".to_string()) {
    Ok(message) => {
      // Delete source data.
      match delete_by_where(session, id, "source_data".to_string(), "source".to_string()) {
        Ok(message_sd) => {
          HttpResponse::Ok().json(Response {
            message: format!("{}\n{}", message, message_sd),
            status: true
          })
        },
        Err((error, status)) => handle_req_error(error, status)
      }
    },
    Err((error, status)) => handle_req_error(error, status)
  }
}

#[delete("{source_id}/data")]
async fn delete_source_data(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  source_id: web::Path<String>,
  params: Option<web::Query<DataInterval>>
) -> HttpResponse {
  let id: String = source_id.to_string();

  match params {
    Some(p) => {
      let mut delete_query = format!("DELETE FROM source_data WHERE source = {}", id);

      if p.since.is_some() {
        delete_query.push_str(format!(" AND stamp >= {}", p.since.unwrap()).as_str());
      }

      if p.until.is_some() {
        delete_query.push_str(format!(" AND stamp <= {}", p.until.unwrap()).as_str());
      }

      if (p.since.is_none() && p.until.is_none() && p.force.is_some() && p.force.unwrap()) || p.since.is_some() || p.until.is_some() {
        let r = session.query(delete_query);

        match r {
          Ok(_) => HttpResponse::Ok().json(Response {
            message: format!("Deleted source data from source {}.", id),
            status: true
          }),
          Err(_) => handle_req_error("Error deleting source data.".to_string(), 500)
        }
      } else {
        handle_req_error("Invalid input request.".to_string(), 400)
      }
    },
    None => handle_req_error("Invalid input request.".to_string(), 400)
  }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(get_source);
  cfg.service(put_source);
  cfg.service(delete_source);
  cfg.service(delete_source_data);
}
