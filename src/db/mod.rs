use cdrs::query::*;
use cdrs::frame::traits::TryFromRow;

use uuid::Uuid;
use blob_uuid::to_uuid;

use std::sync::Arc;
use std::env;

use actix_web::{web};

use crate::common::models::twin::{Source, Element};
use crate::common::models::app::{CurrentSession};

pub fn get_by_id<T: TryFromRow>(session: web::Data<Arc<CurrentSession>>, item_id: String, table: String) -> Result<T, (String, usize)> {
  let id: Uuid;
  
  match Uuid::parse_str(&item_id) {
    Ok(_id) => { id = _id },
    Err(_error) => {
      match to_uuid(&item_id) {
        Ok(_id) => { id = _id },
        Err(_) => { return Err((format!("Invalid input."), 400)); }
      }
    }
  }

  let r = session.query(format!("SELECT * FROM {} WHERE id = {}", table, id));

  let rows = r.expect("Get item by id")
    .get_body().unwrap()
    .into_rows().unwrap();

  if rows.is_empty() {
    return Err(("No item found.".to_string(), 404));
  }
  return Ok(T::try_from_row(rows[0].clone()).unwrap());
}

#[allow(dead_code)]
pub fn delete_by_id<T: TryFromRow>(session: web::Data<Arc<CurrentSession>>, item_id: String, table: String) -> Result<String, (String, usize)> {
  let id: Uuid;

  match Uuid::parse_str(&item_id) {
    Ok(_id) => { id = _id },
    Err(_error) => {
      match to_uuid(&item_id) {
        Ok(_id) => { id = _id },
        Err(_) => { return Err((format!("Invalid input."), 400)); }
      }
    }
  }

  let r = session.query(format!("DELETE FROM {} WHERE id = {}", table, id));

  return match r {
    Ok(_) => Ok(format!("Deleted {} {}.", table, id)),
    Err(_) => Ok(format!("Error deleting {} {}.", table, id))
  };
}

#[allow(dead_code)]
pub fn get_element_sources(session: web::Data<Arc<CurrentSession>>, element_id: String) -> Result<Vec<Source>, (String, usize)> {
  let id: Uuid;

  match Uuid::parse_str(&element_id) {
    Ok(_id) => { id = _id },
    Err(_error) => {
      match to_uuid(&element_id) {
        Ok(_id) => { id = _id },
        Err(_) => { return Err((format!("Invalid input."), 400)); }
      }
    }
  }

  let r = session.query(format!("SELECT * FROM source WHERE element = {}", id));

  let rows = r.expect("Get sources by element")
    .get_body().unwrap()
    .into_rows().unwrap();

  if rows.is_empty() {
    return Ok(vec![]);
  }

  let mut sources: Vec<Source> = Vec::new();
  for row in rows {
    sources.push(Source::try_from_row(row).unwrap());
  }
  Ok(sources)
}

#[allow(dead_code)]
pub fn get_twin_elements(session: web::Data<Arc<CurrentSession>>) -> Result<Vec<Element>, (String, usize)> {
  let twin = env::var("TWIN_INSTANCE").unwrap();

  let r = session.query(format!("SELECT * FROM element WHERE twin = {}", twin));

  let rows = r.expect("Get elements of twin")
    .get_body().unwrap()
    .into_rows().unwrap();

  if rows.is_empty() {
    return Ok(vec![]);
  }

  let mut elements: Vec<Element> = Vec::new();
  for row in rows {
    elements.push(Element::try_from_row(row).unwrap());
  }
  Ok(elements)
}
