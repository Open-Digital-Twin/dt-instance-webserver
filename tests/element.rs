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
use common::models::twin::{Element, ElementRegister};
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

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_body: DataResponse<Element> = resp_2.json().unwrap();
  assert_eq!(resp_2_body.status, true);
  assert_eq!(resp_2_body.data.name, element_register_1.name);
  assert_eq!(resp_2_body.data.parent, element_register_1.parent);

  // Create element with the same name is allowed,
  // but generates a new element, with a different id.
  let resp_3 = put("element").bearer_auth(&token)
    .json(&element_register_1).send().unwrap();

  assert_eq!(resp_3.status(), StatusCode::OK);

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

  assert_eq!(resp_4.status(), StatusCode::OK);

  let resp_4_body: DataResponse<Element> = resp_4.json().unwrap();
  assert_eq!(resp_4_body.status, true);
  assert_eq!(resp_4_body.data.name, element_register_2.name);
  assert_eq!(resp_4_body.data.parent, element_register_2.parent);
  assert_eq!(resp_4_body.data.parent, Some(resp_2_body.data.id));

}
