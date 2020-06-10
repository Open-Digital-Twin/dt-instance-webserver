#[macro_use]
#[allow(unused_imports)]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;

use reqwest::StatusCode;
use serde_json::json;

use crate::cdrs::query::QueryExecutor;

#[cfg(test)]
mod common;
use common::models::response::{Response, LoginResponse};
use common::models::twin::{ElementRegister};
use common::models::user::{Register, UserLogin};

#[test]
/// Register new element for user.
fn create_element() {
  let session = common::get_db_session();
  
  // Create user
  let user_1 = Register {
    email: "example@example.com".to_string(),
    name: "Example user name".to_string(),
    password: "password123".to_string()
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
    name: "Element 1".to_string(),
    parent: None
  };

  let resp_2 = common::request_put("element").bearer_auth(&token)
    .json(&element_register_1).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_body: Response = resp_2.json().unwrap();
  assert_eq!(resp_2_body.status, true);

  // Create element with the same name is allowed
  let resp_3 = common::request_put("element").bearer_auth(&token)
    .json(&element_register_1).send().unwrap();

  assert_eq!(resp_3.status(), StatusCode::OK);

  let resp_3_body: Response = resp_3.json().unwrap();
  assert_eq!(resp_3_body.status, true);

  // Create element with parent element
  let element_register_2 = ElementRegister {
    name: "Element 2".to_string(),
    parent: None
    // parent: element_1.id
  };

  let resp_4 = common::request_put("element").bearer_auth(&token)
    .json(&element_register_2).send().unwrap();

  assert_eq!(resp_4.status(), StatusCode::OK);

  let resp_4_body: Response = resp_4.json().unwrap();
  assert_eq!(resp_4_body.status, true);
}
