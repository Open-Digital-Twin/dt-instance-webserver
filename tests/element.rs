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
use common::requests::{get, post, put, delete};

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

  // Get of created element
  let resp_2_el = get(format!("element/{}", element.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_2_el.status(), StatusCode::OK);

  let resp_2_el_body: DataResponse<Element> = resp_2_el.json().unwrap();
  assert_eq!(resp_2_el_body.status, true);
  assert_eq!(resp_2_el_body.data.name, element_register_1.name);
  assert_eq!(resp_2_el_body.data.parent, element_register_1.parent);
  assert_eq!(resp_2_el_body.data, element);

  // Register sources in element
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

#[test]
fn delete_element() {
  let session = get_db_session();

  // Create user
  let user_1 = Register {
    email: "example_getsources@dominio.com.br".to_string(),
    name: "Example user name".to_string(),
    password: "pass_word".to_string()
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
    name: "Test element to be deleted".to_string(),
    parent: None
  };

  let resp_2 = put("element").bearer_auth(&token).json(&element_register_1).send().unwrap();
  assert_eq!(resp_2.status(), StatusCode::CREATED);

  let resp_2_body: DataResponse<Element> = resp_2.json().unwrap();
  assert_eq!(resp_2_body.status, true);
  assert_eq!(resp_2_body.data.name, element_register_1.name);
  assert_eq!(resp_2_body.data.parent, element_register_1.parent);

  let element = resp_2_body.data;

  // Create data source in element
  let source_register_1 = SourceRegister {
    name: "Source from deleted element".to_string(),
    element: element.id
  };

  let resp_3 = put("source").bearer_auth(&token).json(&source_register_1).send().unwrap();
  assert_eq!(resp_3.status(), StatusCode::CREATED);

  let resp_3_body: DataResponseWithTopics<Source> = resp_3.json().unwrap();
  assert_eq!(resp_3_body.status, true);

  let source_1 = resp_3_body.data;
  assert_eq!(source_1.name, source_register_1.name);
  assert_eq!(source_1.element, source_register_1.element);
  assert_eq!(source_1.element, element.id);

  let resp_4 = put("source").bearer_auth(&token).json(&source_register_1).send().unwrap();
  assert_eq!(resp_4.status(), StatusCode::CREATED);

  let resp_4_body: DataResponseWithTopics<Source> = resp_4.json().unwrap();
  assert_eq!(resp_4_body.status, true);

  let source_2 = resp_4_body.data;
  assert_eq!(source_2.name, source_register_1.name);
  assert_eq!(source_2.element, source_register_1.element);
  assert_eq!(source_2.element, element.id);

  // Insert source data into source history.
  session.query(format!(
    "INSERT INTO source_data (source, stamp, value, created_at) VALUES ({}, toTimestamp(now()), 'value1', toTimestamp(now()))",
    source_1.id
  )).unwrap();

  session.query(format!(
    "INSERT INTO source_data (source, stamp, value, created_at) VALUES ({}, toTimestamp(now()), 'value2', toTimestamp(now()))",
    source_2.id
  )).unwrap();

  let source_data_rows = session.query(format!("SELECT * FROM source_data WHERE source IN ({}, {})", source_1.id, source_2.id)).unwrap()
    .get_body().unwrap()
    .into_rows().unwrap();

  assert_eq!(source_data_rows.len(), 2);

  // Get created element
  let resp_created_element = get(format!("element/{}", element.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_created_element.status(), StatusCode::OK);

  let resp_created_element_body: DataResponse<Element> = resp_created_element.json().unwrap();
  assert!(resp_created_element_body.status);
  assert_eq!(resp_created_element_body.data, element);

  // Get sources by created element
  let resp_created_sources = get(format!("element/{}/sources", element.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_created_sources.status(), StatusCode::OK);

  let resp_created_sources_body: VecDataResponse<Source> = resp_created_sources.json().unwrap();
  let obtained_sources = resp_created_sources_body.data;
  assert_eq!(obtained_sources.len(), 2);

  let obtained_source_1 = obtained_sources.iter().find(|&source| source.id == source_1.id).unwrap();
  let obtained_source_2 = obtained_sources.iter().find(|&source| source.id == source_2.id).unwrap();

  assert_eq!(*obtained_source_1, source_1);
  assert_eq!(*obtained_source_2, source_2);

  // Get sources by id
  let resp_source_1 = get(format!("source/{}", source_1.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_1.status(), StatusCode::OK);
  let resp_source_1_body: DataResponse<Source> = resp_source_1.json().unwrap();
  let source_1_by_id = resp_source_1_body.data;
  assert_eq!(source_1_by_id, source_1);

  let resp_source_2 = get(format!("source/{}", source_2.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_2.status(), StatusCode::OK);
  let resp_source_2_body: DataResponse<Source> = resp_source_2.json().unwrap();
  let source_2_by_id = resp_source_2_body.data;
  assert_eq!(source_2_by_id, source_2);

  // Delete element
  let resp_delete_element = delete(format!("element/{}", element.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_delete_element.status(), StatusCode::OK);

  let resp_delete_element_body: Response = resp_delete_element.json().unwrap();
  assert!(resp_delete_element_body.status);

  // Get of deleted element should fail
  let resp_created_element_deleted = get(format!("element/{}", element.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_created_element_deleted.status(), StatusCode::NOT_FOUND);

  let resp_created_element_deleted_body: Response = resp_created_element_deleted.json().unwrap();
  assert_eq!(resp_created_element_deleted_body.status, false);

  // Get of deleted elements from source should return no sources
  let resp_created_sources_deleted = get(format!("element/{}/sources", element.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_created_sources_deleted.status(), StatusCode::OK);

  let resp_created_sources_body_deleted: VecDataResponse<Source> = resp_created_sources_deleted.json().unwrap();
  let obtained_sources_2 = resp_created_sources_body_deleted.data;
  assert_eq!(obtained_sources_2.len(), 0);

  // Get of deleted sources should fail
  let resp_source_1_deleted = get(format!("source/{}", source_1.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_1_deleted.status(), StatusCode::NOT_FOUND);
  let resp_source_1_deleted_body: Response = resp_source_1_deleted.json().unwrap();
  assert_eq!(resp_source_1_deleted_body.status, false);

  let resp_source_2_deleted = get(format!("source/{}", source_2.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_2_deleted.status(), StatusCode::NOT_FOUND);
  let resp_source_2_deleted_body: Response = resp_source_2_deleted.json().unwrap();
  assert_eq!(resp_source_2_deleted_body.status, false);

  // Data of deleted sources should be deleted
  let deleted_source_data_rows = session.query(format!("SELECT * FROM source_data WHERE source IN ({}, {})", source_1.id, source_2.id)).unwrap()
    .get_body().unwrap()
    .into_rows().unwrap();

  assert_eq!(deleted_source_data_rows.len(), 0);
}
