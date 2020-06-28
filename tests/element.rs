#[macro_use]
#[allow(unused_imports)]
extern crate cdrs;

#[macro_use]
#[allow(unused_imports)]
extern crate cdrs_helpers_derive;

use reqwest::StatusCode;

#[allow(unused_imports)]
use crate::cdrs::query::QueryExecutor;

#[cfg(test)]
mod common;
use common::models::response::{Response, LoginResponse, DataResponse, DataResponseWithTopics, VecDataResponse};
use common::models::twin::{Element, ElementRegister, Source, SourceRegister};
use common::models::user::{Register, UserLogin};
use common::db::get_db_session;
use common::requests::{get, post, put};


#[test]
/// Register new element for user.
fn create_element() {
  let session = get_db_session();
  
  // Create user
  let user_1 = Register {
    email: "example@example.com".to_string(),
    name: "Example user name".to_string(),
    password: "password123".to_string()
  };
  session.query(format!("DELETE FROM user WHERE email='{}'", user_1.email).to_string()).unwrap();
  
  let resp_1 = post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);
  let resp_1_body: Response = resp_1.json().unwrap();
  assert_eq!(resp_1_body.status, true);

  let login = UserLogin {
    email: user_1.email.to_string(),
    password: user_1.password.to_string(),
    remember_me: false
  };

  let resp_login = post("user/login")
    .json(&login).send().unwrap();

  assert_eq!(resp_login.status(), StatusCode::OK);

  let resp_login_json: LoginResponse = resp_login.json().unwrap();
  assert!(resp_login_json.status);
  let token = resp_login_json.token;

  // Create element
  let element_register_1 = ElementRegister {
    name: "Element 1".to_string(),
    parent: None
  };

  let resp_2 = put("element").bearer_auth(&token)
    .json(&element_register_1).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::CREATED);

  let resp_2_body: DataResponse<Element> = resp_2.json().unwrap();
  assert_eq!(resp_2_body.status, true);
  assert_eq!(resp_2_body.data.name, element_register_1.name);
  assert_eq!(resp_2_body.data.parent, element_register_1.parent);

  // Create element with the same name is allowed,
  // but generates a new element, with a different id.
  let resp_3 = put("element").bearer_auth(&token)
    .json(&element_register_1).send().unwrap();

  assert_eq!(resp_3.status(), StatusCode::CREATED);

  let resp_3_body: DataResponse<Element> = resp_3.json().unwrap();
  assert_eq!(resp_3_body.status, true);
  assert_eq!(resp_3_body.data.name, element_register_1.name);
  assert_eq!(resp_3_body.data.parent, element_register_1.parent);

  // Tests if two equal created elements have different identificators
  assert_ne!(resp_3_body.data.id, resp_2_body.data.id);

  // Create element with another element as parent
  let element_register_2 = ElementRegister {
    name: "Element 2".to_string(),
    parent: Some(resp_2_body.data.id)
  };

  let resp_4 = put("element").bearer_auth(&token)
    .json(&element_register_2).send().unwrap();

  assert_eq!(resp_4.status(), StatusCode::CREATED);

  let resp_4_body: DataResponse<Element> = resp_4.json().unwrap();
  assert_eq!(resp_4_body.status, true);
  assert_eq!(resp_4_body.data.name, element_register_2.name);
  assert_eq!(resp_4_body.data.parent, element_register_2.parent);
  assert_eq!(resp_4_body.data.parent, Some(resp_2_body.data.id));

  // Test GET of element with no parent
  let resp_element = get(format!("element/{}", resp_2_body.data.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_element.status(), StatusCode::OK);

  let resp_element_body: DataResponse<Element> = resp_element.json().unwrap();
  assert_eq!(resp_element_body.status, true);
  assert_eq!(resp_element_body.data.id, resp_2_body.data.id);
  assert_eq!(resp_element_body.data.name, resp_2_body.data.name);
  assert_eq!(resp_element_body.data.parent, resp_2_body.data.parent);
  assert_eq!(resp_element_body.data.twin, resp_2_body.data.twin);
  assert_eq!(resp_element_body.data, resp_2_body.data);

  // Test GET of element with parent
  let resp_element2 = get(format!("element/{}", resp_4_body.data.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_element2.status(), StatusCode::OK);

  let resp_element2_body: DataResponse<Element> = resp_element2.json().unwrap();
  assert_eq!(resp_element2_body.status, true);
  assert_eq!(resp_element2_body.data.id, resp_4_body.data.id);
  assert_eq!(resp_element2_body.data.name, resp_4_body.data.name);
  assert_eq!(resp_element2_body.data.parent, resp_4_body.data.parent);
  assert_eq!(resp_element2_body.data.twin, resp_4_body.data.twin);
  assert_eq!(resp_element2_body.data, resp_4_body.data);
}

#[test]
fn get_element_sources() {
  let session = get_db_session();

  // Create user
  let user_1 = Register {
    email: "example_getsources@example.com".to_string(),
    name: "Example user name".to_string(),
    password: "password123".to_string()
  };
  session.query(format!("DELETE FROM user WHERE email='{}'", user_1.email).to_string()).unwrap();

  let resp_1 = post("user/register").json(&user_1).send().unwrap();
  assert_eq!(resp_1.status(), StatusCode::OK);
  let resp_1_body: Response = resp_1.json().unwrap();
  assert_eq!(resp_1_body.status, true);

  let login = UserLogin {
    email: user_1.email.to_string(),
    password: user_1.password.to_string(),
    remember_me: false
  };

  let resp_login = post("user/login").json(&login).send().unwrap();
  assert_eq!(resp_login.status(), StatusCode::OK);
  let resp_login_json: LoginResponse = resp_login.json().unwrap();
  assert!(resp_login_json.status);

  let token = resp_login_json.token;

  // Create element
  let element_register_1 = ElementRegister {
    name: "Element with Sources".to_string(),
    parent: None
  };

  let resp_2 = put("element").bearer_auth(&token).json(&element_register_1).send().unwrap();
  assert_eq!(resp_2.status(), StatusCode::CREATED);

  let resp_2_body: DataResponse<Element> = resp_2.json().unwrap();
  assert_eq!(resp_2_body.status, true);
  assert_eq!(resp_2_body.data.name, element_register_1.name);
  assert_eq!(resp_2_body.data.parent, element_register_1.parent);

  let element = resp_2_body.data;

  let source_register = SourceRegister {
    name: "Source".to_string(),
    element: element.id
  };

  let resp_source_1 = put("source").bearer_auth(&token).json(&source_register).send().unwrap();
  assert_eq!(resp_source_1.status(), StatusCode::CREATED);
  let resp_source_1_body: DataResponseWithTopics<Source> = resp_source_1.json().unwrap();

  let resp_source_2 = put("source").bearer_auth(&token).json(&source_register).send().unwrap();
  assert_eq!(resp_source_2.status(), StatusCode::CREATED);
  let resp_source_2_body: DataResponseWithTopics<Source> = resp_source_2.json().unwrap();

  let resp_source_3 = put("source").bearer_auth(&token).json(&source_register).send().unwrap();
  assert_eq!(resp_source_3.status(), StatusCode::CREATED);
  let resp_source_3_body: DataResponseWithTopics<Source> = resp_source_3.json().unwrap();

  let source_1 = resp_source_1_body.data;
  let source_2 = resp_source_2_body.data;
  let source_3 = resp_source_3_body.data;

  // Get element sources
  let resp_sources = get(format!("element/{}/sources", element.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_sources.status(), StatusCode::OK);

  let resp_sources_body: VecDataResponse<Source> = resp_sources.json().unwrap();
  let obtained_sources = resp_sources_body.data;
  assert_eq!(obtained_sources.len(), 3);

  let obtained_source_1 = obtained_sources.iter().find(|&source| source.id == source_1.id).unwrap();
  let obtained_source_2 = obtained_sources.iter().find(|&source| source.id == source_2.id).unwrap();
  let obtained_source_3 = obtained_sources.iter().find(|&source| source.id == source_3.id).unwrap();

  assert_eq!(obtained_source_1.id, source_1.id);
  assert_eq!(obtained_source_1.name, source_1.name);
  assert_eq!(obtained_source_1.element, source_1.element);
  assert_eq!(*obtained_source_1, source_1);
  assert_eq!(obtained_source_1.element, element.id);

  assert_eq!(obtained_source_2.id, source_2.id);
  assert_eq!(obtained_source_2.name, source_2.name);
  assert_eq!(obtained_source_2.element, source_2.element);
  assert_eq!(*obtained_source_2, source_2);
  assert_eq!(obtained_source_2.element, element.id);

  assert_eq!(obtained_source_3.id, source_3.id);
  assert_eq!(obtained_source_3.name, source_3.name);
  assert_eq!(obtained_source_3.element, source_3.element);
  assert_eq!(*obtained_source_3, source_3);
  assert_eq!(obtained_source_3.element, element.id);
}
