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
use common::models::response::{Response, LoginResponse, DataResponse, DataResponseWithTopics};
use common::models::twin::{Element, ElementRegister, Source, SourceRegister};
use common::models::user::{Register, UserLogin};
use common::models::app::{SOURCE_DATA_ACK_TOPIC,SOURCE_DATA_TOPIC};
use common::db::get_db_session;
use common::requests::{get, post, put, delete};

#[test]
/// Register new source of element.
fn create_source() {
  let session = get_db_session();
  
  // Create user
  let user_1 = Register {
    email: "example10@example.co.uk".to_string(),
    name: "Name of user".to_string(),
    password: "123qwerty123".to_string()
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
    name: "Element #1".to_string(),
    parent: None
  };

  let resp_2 = put("element").bearer_auth(&token)
    .json(&element_register_1).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::CREATED);

  let resp_2_body: DataResponse<Element> = resp_2.json().unwrap();
  assert_eq!(resp_2_body.status, true);
  assert_eq!(resp_2_body.data.name, element_register_1.name);
  assert_eq!(resp_2_body.data.parent, element_register_1.parent);

  // Create data source in element
  let source_register_1 = SourceRegister {
    name: "Source #1".to_string(),
    element: resp_2_body.data.id
  };

  let resp_3 = put("source").bearer_auth(&token)
    .json(&source_register_1).send().unwrap();

  assert_eq!(resp_3.status(), StatusCode::CREATED);

  let resp_3_body: DataResponseWithTopics<Source> = resp_3.json().unwrap();
  assert_eq!(resp_3_body.status, true);
  assert_eq!(resp_3_body.data.name, source_register_1.name);
  assert_eq!(resp_3_body.data.element, source_register_1.element);
  assert_eq!(resp_3_body.data.element, resp_2_body.data.id);

  assert!(resp_3_body.topics.contains_key(SOURCE_DATA_TOPIC));

  let source_topic: Vec<&str> = resp_3_body.topics.get(SOURCE_DATA_TOPIC).unwrap().split('/').collect();
  assert_eq!(source_topic.len(), 3);

  // let twin_id = source_topic[0];
  let element_id = source_topic[1];
  let source_id = source_topic[2];

  // TODO: use source_topic[0] to get Twin
  
  // Get element from created source topic
  let resp_element = get(format!("element/{}", element_id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_element.status(), StatusCode::OK);

  let resp_element_body: DataResponse<Element> = resp_element.json().unwrap();
  assert_eq!(resp_element_body.status, true);
  assert_eq!(resp_element_body.data.id, resp_2_body.data.id);
  assert_eq!(resp_element_body.data.name, resp_2_body.data.name);
  assert_eq!(resp_element_body.data.parent, resp_2_body.data.parent);
  assert_eq!(resp_element_body.data.twin, resp_2_body.data.twin);
  assert_eq!(resp_element_body.data, resp_2_body.data);

  // Get source from created id topic
  // Test source obtained from GET to the one received on source creation.
  let resp_source = get(format!("source/{}", source_id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source.status(), StatusCode::OK);

  let resp_source_body: DataResponse<Source> = resp_source.json().unwrap();
  assert_eq!(resp_source_body.status, true);
  assert_eq!(resp_source_body.data.id, resp_3_body.data.id);
  assert_eq!(resp_source_body.data.name, resp_3_body.data.name);
  assert_eq!(resp_source_body.data.element, resp_3_body.data.element);
  assert_eq!(resp_source_body.data, resp_3_body.data);

  // Get source from created source object id
  let resp_source2 = get(format!("source/{}", resp_3_body.data.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source2.status(), StatusCode::OK);

  let resp_source_body2: DataResponse<Source> = resp_source2.json().unwrap();
  assert_eq!(resp_source_body2.status, true);
  assert_eq!(resp_source_body2.data.id, resp_3_body.data.id);
  assert_eq!(resp_source_body2.data.name, resp_3_body.data.name);
  assert_eq!(resp_source_body2.data.element, resp_3_body.data.element);
  assert_eq!(resp_source_body2.data, resp_3_body.data);

  // Get source using invalid input
  let resp_source_invalid = get("source/123").bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_invalid.status(), StatusCode::BAD_REQUEST);
  let resp_source_invalid_body: Response = resp_source_invalid.json().unwrap();
  assert_eq!(resp_source_invalid_body.status, false);

  // Get source using invalid id
  let resp_source_invalid = get(format!("source/{}-123", source_id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_invalid.status(), StatusCode::BAD_REQUEST);
  let resp_source_invalid_body: Response = resp_source_invalid.json().unwrap();
  assert_eq!(resp_source_invalid_body.status, false);

  assert!(resp_3_body.topics.contains_key(SOURCE_DATA_ACK_TOPIC));
  let source_ack_topic: Vec<&str> = resp_3_body.topics.get(SOURCE_DATA_ACK_TOPIC).unwrap().split('/').collect();
  assert_eq!(source_ack_topic.len(), 4);

  // TODO: use source_ack_topic[0] to get Twin

  let resp_element = get(format!("element/{}", source_ack_topic[1]).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_element.status(), StatusCode::OK);

  let resp_element_body: DataResponse<Element> = resp_element.json().unwrap();
  assert_eq!(resp_element_body.status, true);
  assert_eq!(resp_element_body.data.id, resp_2_body.data.id);
  assert_eq!(resp_element_body.data.name, resp_2_body.data.name);
  assert_eq!(resp_element_body.data.parent, resp_2_body.data.parent);
  assert_eq!(resp_element_body.data.twin, resp_2_body.data.twin);
  assert_eq!(resp_element_body.data, resp_2_body.data);

  let resp_source_2 = get(format!("source/{}", source_ack_topic[2]).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_2.status(), StatusCode::OK);

  let resp_source_2_body: DataResponse<Source> = resp_source_2.json().unwrap();
  assert_eq!(resp_source_2_body.status, true);
  assert_eq!(resp_source_2_body.data.id, resp_3_body.data.id);
  assert_eq!(resp_source_2_body.data.name, resp_3_body.data.name);
  assert_eq!(resp_source_2_body.data.element, resp_3_body.data.element);
  assert_eq!(resp_source_2_body.data, resp_3_body.data);
}


#[test]
/// Delete a created source.
fn delete_source() {
  let session = get_db_session();

  let user_1 = Register {
    email: "example11@example.co.uk".to_string(),
    name: "User name".to_string(),
    password: "qWeRtY123$".to_string()
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
    name: "Element who will have no source".to_string(),
    parent: None
  };

  let resp_2 = put("element").bearer_auth(&token)
    .json(&element_register_1).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::CREATED);

  let resp_2_body: DataResponse<Element> = resp_2.json().unwrap();
  assert_eq!(resp_2_body.status, true);

  let element = resp_2_body.data;
  assert_eq!(element.name, element_register_1.name);
  assert_eq!(element.parent, element_register_1.parent);

  // Create data source in element
  let source_register_1 = SourceRegister {
    name: "Source to be deleted".to_string(),
    element: element.id
  };

  let resp_3 = put("source").bearer_auth(&token)
    .json(&source_register_1).send().unwrap();

  assert_eq!(resp_3.status(), StatusCode::CREATED);

  let resp_3_body: DataResponseWithTopics<Source> = resp_3.json().unwrap();
  assert_eq!(resp_3_body.status, true);

  let source = resp_3_body.data;
  assert_eq!(source.name, source_register_1.name);
  assert_eq!(source.element, source_register_1.element);
  assert_eq!(source.element, element.id);

  // Get source from created source object id
  let resp_source = get(format!("source/{}", source.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source.status(), StatusCode::OK);

  let resp_source_body: DataResponse<Source> = resp_source.json().unwrap();
  assert_eq!(resp_source_body.status, true);
  assert_eq!(resp_source_body.data.id, source.id);
  assert_eq!(resp_source_body.data.name, source.name);
  assert_eq!(resp_source_body.data.element, source.element);
  assert_eq!(resp_source_body.data, source);

  // Insert source data into source history.
  session.query(format!(
    "INSERT INTO source_data (source, stamp, value, created_at) VALUES ({}, toTimestamp(now()), 'value1', toTimestamp(now()))",
    source.id
  )).unwrap();

  session.query(format!(
    "INSERT INTO source_data (source, stamp, value, created_at) VALUES ({}, toTimestamp(now()), 'value2', toTimestamp(now()))",
    source.id
  )).unwrap();

  let source_data_rows = session.query(format!("SELECT * FROM source_data WHERE source = {}", source.id)).unwrap()
    .get_body().unwrap()
    .into_rows().unwrap();

  assert_eq!(source_data_rows.len(), 2);

  // Delete created source
  let resp_delete = delete(format!("source/{}", source.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_delete.status(), StatusCode::OK);

  let resp_delete_body: Response = resp_delete.json().unwrap();
  assert_eq!(resp_delete_body.status, true);

  // Get deleted source does not fail, but returns OK.
  let resp_source_deleted = get(format!("source/{}", source.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_deleted.status(), StatusCode::NOT_FOUND);

  let resp_source_deleted_body: Response = resp_source_deleted.json().unwrap();
  assert_eq!(resp_source_deleted_body.status, false);

  // Delete on deleted source should not fail
  let resp_source_deleted2 = delete(format!("source/{}", source.id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_deleted2.status(), StatusCode::OK);

  let resp_source_deleted2_body: Response = resp_source_deleted2.json().unwrap();
  assert_eq!(resp_source_deleted2_body.status, true);

  // Source data from deleted source should be deleted.
  let source_data_rows2 = session.query(format!("SELECT * FROM source_data WHERE source = {}", source.id)).unwrap()
    .get_body().unwrap()
    .into_rows().unwrap();

  assert_eq!(source_data_rows2.len(), 0);

  // Delete on invalid input should fail
  let resp_source_deleted3 = delete("source/123").bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_deleted3.status(), StatusCode::BAD_REQUEST);

  let resp_source_deleted3_body: Response = resp_source_deleted3.json().unwrap();
  assert_eq!(resp_source_deleted3_body.status, false);
}
