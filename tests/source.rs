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
use common::requests::{get, post, put};

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

  assert_eq!(resp_2.status(), StatusCode::OK);

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

  assert_eq!(resp_3.status(), StatusCode::OK);

  let resp_3_body: DataResponseWithTopics<Source> = resp_3.json().unwrap();
  assert_eq!(resp_3_body.status, true);
  assert_eq!(resp_3_body.data.name, source_register_1.name);
  assert_eq!(resp_3_body.data.element, source_register_1.element);
  assert_eq!(resp_3_body.data.element, resp_2_body.data.id);

  assert!(resp_3_body.topics.contains_key(SOURCE_DATA_TOPIC));

  let source_topic: Vec<&str> = resp_3_body.topics.get(SOURCE_DATA_TOPIC).unwrap().split('/').collect();
  assert_eq!(source_topic.len(), 3);

  // let twin_id = source_topic[0];
  // let element_id = source_topic[1];
  let source_id = source_topic[2];

  // TODO: use source_topic[0] to get Twin
  // TODO: use source_topic[1] to get Element

  // Get source from created id
  // Test source obtained from GET to the one received on source creation.
  let resp_source = get(format!("source/{}", source_id).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source.status(), StatusCode::OK);

  let resp_source_body: DataResponse<Source> = resp_source.json().unwrap();
  assert_eq!(resp_source_body.status, true);
  assert_eq!(resp_source_body.data.id, resp_3_body.data.id);
  assert_eq!(resp_source_body.data.name, resp_3_body.data.name);
  assert_eq!(resp_source_body.data.element, resp_3_body.data.element);
  assert_eq!(resp_source_body.data, resp_3_body.data);

  // Get invalid source
  let resp_source_invalid = get("source/123").bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_invalid.status(), StatusCode::NOT_FOUND);
  let resp_source_invalid_body: Response = resp_source_invalid.json().unwrap();
  assert_eq!(resp_source_invalid_body.status, false);

  assert!(resp_3_body.topics.contains_key(SOURCE_DATA_ACK_TOPIC));
  let source_ack_topic: Vec<&str> = resp_3_body.topics.get(SOURCE_DATA_ACK_TOPIC).unwrap().split('/').collect();
  assert_eq!(source_ack_topic.len(), 4);

  // TODO: use source_ack_topic[0] to get Twin
  // TODO: use source_ack_topic[1] to get Element

  let resp_source_2 = get(format!("source/{}", source_ack_topic[2]).as_str()).bearer_auth(&token).send().unwrap();
  assert_eq!(resp_source_2.status(), StatusCode::OK);

  let resp_source_2_body: DataResponse<Source> = resp_source_2.json().unwrap();
  assert_eq!(resp_source_2_body.status, true);
  assert_eq!(resp_source_2_body.data.id, resp_3_body.data.id);
  assert_eq!(resp_source_2_body.data.name, resp_3_body.data.name);
  assert_eq!(resp_source_2_body.data.element, resp_3_body.data.element);
  assert_eq!(resp_source_2_body.data, resp_3_body.data);
}


