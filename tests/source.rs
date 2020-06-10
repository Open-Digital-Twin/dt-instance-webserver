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
use common::models::response::{Response, LoginResponse, DataResponse};
use common::models::twin::{Element, ElementRegister, Source, SourceRegister};
use common::models::user::{Register, UserLogin};

#[test]
/// Register new source of element.
fn create_source() {
  let session = common::get_db_session();
  
  // Create user
  let user_1 = Register {
    email: "example10@example.co.uk".to_string(),
    name: "Name of user".to_string(),
    password: "123qwerty123".to_string()
  };
  session.query(format!("DELETE FROM user WHERE email='{}'", user_1.email).to_string()).unwrap();
  
  let resp_1 = common::request_post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);
  let resp_1_body: Response = resp_1.json().unwrap();
  assert_eq!(resp_1_body.status, true);

  let login = UserLogin {
    email: user_1.email.to_string(),
    password: user_1.password.to_string(),
    remember_me: false
  };

  let resp_login = common::request_post("user/login")
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

  let resp_2 = common::request_put("element").bearer_auth(&token)
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

  let resp_3 = common::request_put("source").bearer_auth(&token)
    .json(&source_register_1).send().unwrap();

  assert_eq!(resp_3.status(), StatusCode::OK);

  let resp_3_body: DataResponse<Source> = resp_3.json().unwrap();
  assert_eq!(resp_3_body.status, true);
  assert_eq!(resp_3_body.data.name, source_register_1.name);
  assert_eq!(resp_3_body.data.element, source_register_1.element);
  assert_eq!(resp_3_body.data.element, resp_2_body.data.id);

  resp_3_body.data.to_query();
}
