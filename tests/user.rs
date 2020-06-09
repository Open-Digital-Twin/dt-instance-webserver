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
use common::models::user::{Register, UserLogin};

#[test]
fn create_user() {
  let session = common::get_db_session();
  
  // Create user
  let user_1 = Register {
    email: "example_1@example.com".to_string(),
    name: "Example user".to_string(),
    password: "example_password".to_string()
  };
  session.query(format!("DELETE FROM user WHERE email='{}'", user_1.email).to_string()).unwrap();
  
  let resp_1 = common::request_post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);

  let resp_1_body: Response = resp_1.json().unwrap();

  assert_eq!(resp_1_body.message, format!("Success in creating user {}.", user_1.email));
  assert_eq!(resp_1_body.status, true);

  // Create user with same email should fail
  let resp_2 = common::request_post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_body: Response = resp_2.json().unwrap();

  assert_eq!(resp_2_body.message, format!("User {} already exists.", user_1.email));
  assert_eq!(resp_2_body.status, false);
}

#[test]
/// Register new user, then tests a route that requires authentication.
fn login() {
  let session = common::get_db_session();

  let register = Register {
    email: "example_3@example.com".to_string(),
    name: "Example user".to_string(),
    password: "example_password".to_string()
  };
    
  session.query(format!("DELETE FROM user WHERE email='{}'", register.email).to_string()).unwrap();

  let resp_1 = common::request_post("user/register")
    .json(&register).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);

  let login = UserLogin {
    email: register.email.to_string(),
    password: register.password.to_string(),
    remember_me: false
  };

  let resp_2 = common::request_post("user/login")
    .json(&login).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_json: LoginResponse = resp_2.json().unwrap();
  let token = resp_2_json.token;
  assert!(resp_2_json.status);

  // Test authentication with logged in user
  let resp3 = common::request_get("user/test").send().unwrap();
  assert_eq!(resp3.status(), StatusCode::UNAUTHORIZED);

  let resp4 = common::request_get("user/test").bearer_auth(token).send().unwrap();
  assert_eq!(resp4.status(), StatusCode::OK);
}

#[test]
fn unauthorized() {
  let resp = common::request_get("user/test").send().unwrap();
  assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "Expected unauthorized access in user/test.");
}
