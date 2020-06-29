use cdrs::query::*;
use cdrs::frame::traits::TryFromRow;

use crate::common::models::app::{CurrentSession, Environment};
use crate::common::models::response::{Response, DataResponse, VecDataResponse};
use crate::common::models::twin::*;
use crate::common::models::request::Serializeable;
use crate::common::db::{get_twin_elements, get_element_sources};

use crate::middlewares::auth::AuthValidator;

use std::sync::Arc;

use log::{info};
use actix_web::{get, web, HttpResponse};
use std::cmp::Ordering;

fn sort_elements(a: &Element, b: &Element) -> Ordering {
  if (a.parent.is_none() && b.parent.is_none()) || (a.parent.is_some() && b.parent.is_some()) {
    Ordering::Equal
  } else if a.parent.is_some() {
    Ordering::Greater
  } else {
    Ordering::Less
  }
}

fn handle_parentless_elements(elements: &mut Vec<ElementSerialized>, child_elements: &mut Vec<ElementSerialized>) {
  info!("Next handle {}", child_elements.len());
  if !child_elements.is_empty() {
    let mut next_child_elements: Vec<ElementSerialized> = Vec::new();

    for s_element in child_elements {
      let found_element = (*elements).iter_mut().find(|item| item.element.id == s_element.element.parent.unwrap());
      match found_element {
        Some(el) => {
          el.add_child(s_element.clone());
        },
        None => {
          // Handle on next iteration of function
          next_child_elements.push(s_element.clone());
        }
      }
    }

    // Repeat for remaining children
    if !next_child_elements.is_empty() {
      handle_parentless_elements(elements, &mut next_child_elements);
    }
  }
}

#[get("elements")]
async fn get_elements_of_twin(
  _auth: AuthValidator,
  session: web::Data<Arc<CurrentSession>>,
  params: Option<web::Query<Serializeable>>
) -> HttpResponse {
  match get_twin_elements(session.clone()) {
    Ok(mut elements) => {
      elements.sort_by(sort_elements);
      let count = elements.len();

      match params {
        Some(p) => {
          if p.serialized {
            let mut s_elements: Vec<ElementSerialized> = Vec::new();
            let mut parentless_elements: Vec<ElementSerialized> = Vec::new();

            for element in elements.clone() {
              if element.parent.is_none() {
                s_elements.push(ElementSerialized {
                  element: element.clone(),
                  sources: get_element_sources(session.clone(), element.id.to_string()).unwrap(),
                  children: Vec::new()
                });
              } else {
                let new_element = ElementSerialized {
                  element: element.clone(),
                  sources: get_element_sources(session.clone(), element.id.to_string()).unwrap(),
                  children: Vec::new()
                };

                let found_element = s_elements.iter_mut().find(|item| item.element.id == element.parent.unwrap());
                match found_element {
                  Some(el) => el.add_child(new_element),
                  None => parentless_elements.push(new_element)
                }
              }
            }

            if !parentless_elements.is_empty() {
              handle_parentless_elements(&mut s_elements, &mut parentless_elements);          
            }
            
            return HttpResponse::Ok().json(VecDataResponse {
              message: format!("Found {} elements for twin.", s_elements.len()),
              data: s_elements,
              status: true
            });
          }
        }
        None => {}
      }

      return HttpResponse::Ok().json(VecDataResponse {
        message: format!("Found {} elements for twin.", count),
        data: elements,
        status: true
      });
    },
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
  cfg.service(get_elements_of_twin);
}
