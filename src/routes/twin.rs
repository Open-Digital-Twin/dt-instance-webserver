use crate::common::models::app::{CurrentSession};
use crate::common::models::response::{VecDataResponse};
use crate::common::models::twin::*;
use crate::common::models::request::Serializeable;

use crate::db::{get_twin_elements, get_element_sources};

use crate::middlewares::auth::AuthValidator;

use crate::routes::handle_req_error;

use std::sync::Arc;

use log::{debug};
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

fn find_element(elements: &mut Vec<ElementSerialized>, id: uuid::Uuid, count: usize) -> Option<&mut ElementSerialized> {
  debug!("Finding element {} - elements: {} - find_element iteration {}", id, elements.len(), count);

  if count < 10 {
    for item in elements {
      debug!("Searching in element {}", item.element.id);
      if item.element.id == id {
        debug!("Found {}.", id);
        return Some(item);
      } else if !item.children.is_empty() {
        debug!("Testing children of element: has {}", item.children.len());
        match find_element(&mut item.children, id, count + 1) {
          Some(el) => {
            return Some(el);
          },
          None => {}
        }
      } else {
        debug!("No children on element");
      }
    }
    debug!("Did not find {} in elements", id);
    return None;
  }
  return None;
}

fn handle_parentless_elements(elements: &mut Vec<ElementSerialized>, child_elements: &mut Vec<ElementSerialized>) {
  debug!("Remaining parentless elements {}", child_elements.len());

  if !child_elements.is_empty() {
    let mut next_child_elements: Vec<ElementSerialized> = Vec::new();

    for s_element in child_elements {
      debug!("Parentless current search: {}", s_element.element.id);
      let found_element = find_element(elements, s_element.element.parent.unwrap(), 1);
      match found_element {
        Some(el) => {
          debug!("Found {}. Adding {} to element children.", el.element.id, s_element.element.id);
          (*el).add_child(s_element.clone());
        },
        None => {
          // Handle on next iteration of function
          debug!("Not found {} in child_elements. Adding to next iteration", s_element.element.parent.unwrap());
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

                parentless_elements.push(new_element);
              }
            }

            if !parentless_elements.is_empty() {
              handle_parentless_elements(&mut s_elements, &mut parentless_elements);          
            }
            
            return HttpResponse::Ok().json(VecDataResponse {
              message: format!("Found {} root elements for twin.", s_elements.len()),
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
    Err((error, status)) => handle_req_error(error, status)
  }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(get_elements_of_twin);
}
