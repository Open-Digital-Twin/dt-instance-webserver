use cdrs::query::*;
use cdrs::query_values;
use cdrs::frame::traits::TryFromRow;

use crate::common::models::app::{Environment, SOURCE_DATA_TOPIC, SOURCE_DATA_ACK_TOPIC};
use crate::common::models::response::{Response, DataResponse, DataResponseWithTopics};
use crate::common::models::twin::*;

use crate::{CurrentSession};
use crate::middlewares::auth::AuthValidator;
use std::sync::Arc;

use log::{info};
use actix_web::{get, put, web, HttpResponse};

use std::collections::HashMap;

use uuid::Uuid;
use blob_uuid::to_uuid;

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

fn get_source_by_id(session: web::Data<Arc<CurrentSession>>, source: String) -> Result<Source, (String, usize)> {
  let id: Uuid;
  
  match Uuid::parse_str(&source) {
    Ok(_id) => { id = _id },
    Err(_error) => {
      match to_uuid(&source) {
        Ok(_id) => { id = _id },
        Err(_) => { return Err((format!("Invalid input source."), 400)); }
      }
    }
  }

  let r = session.query(format!("SELECT * FROM source WHERE id = {}", id));

  let rows = r.expect("Get source by id")
    .get_body().unwrap()
    .into_rows().unwrap();

  if rows.is_empty() {
    return Err(("No source found.".to_string(), 404));
  }
  return Ok(Source::try_from_row(rows[0].clone()).unwrap());
}

#[get("{source_id}")]
async fn get_source(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  source_id: web::Path<String>
) -> HttpResponse {
  match get_source_by_id(session, source_id.to_string()) {
    Ok(source) => HttpResponse::Ok().json(DataResponse {
      message: format!("Found source {}", source.clone().id),
      data: source,
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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(get_source);
  cfg.service(put_source);
}
