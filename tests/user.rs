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
use common::models::user::{Register, UserLogin, User};
use common::db::get_db_session;
use common::requests::{get, post};

#[test]
fn create_user() {
  let session = get_db_session();
  
  // Create user
  let user_1 = Register {
    email: "example_1@example.com".to_string(),
    name: "Example user".to_string(),
    password: "example_password".to_string()
  };
  session.query(format!("DELETE FROM user WHERE email='{}'", user_1.email).to_string()).unwrap();
  
  let resp_1 = post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);

  let resp_1_body: DataResponse<User> = resp_1.json().unwrap();

  assert_eq!(resp_1_body.message, format!("Success in creating user {}.", user_1.email));
  assert_eq!(resp_1_body.status, true);
  assert_eq!(resp_1_body.data.email, user_1.email);
  assert_eq!(resp_1_body.data.name, user_1.name);
  assert_ne!(resp_1_body.data.password, user_1.password); // Just in case.

  // Create user with same email should fail
  let resp_2 = post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_body: Response = resp_2.json().unwrap();

  assert_eq!(resp_2_body.message, format!("User {} already exists.", user_1.email));
  assert_eq!(resp_2_body.status, false);
}

#[test]
/// Register new user, then tests a route that requires authentication.
fn login() {
  let session = get_db_session();

  let register = Register {
    email: "example_3@example.com".to_string(),
    name: "Example user".to_string(),
    password: "example_password".to_string()
  };
    
  session.query(format!("DELETE FROM user WHERE email='{}'", register.email).to_string()).unwrap();

  let resp_1 = post("user/register")
    .json(&register).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);

  let login = UserLogin {
    email: register.email.to_string(),
    password: register.password.to_string(),
    remember_me: false
  };

  let resp_2 = post("user/login")
    .json(&login).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_json: LoginResponse = resp_2.json().unwrap();
  let token = resp_2_json.token;
  assert!(resp_2_json.status);

  // Test authentication with logged in user
  let resp3 = get("user/test").send().unwrap();
  assert_eq!(resp3.status(), StatusCode::UNAUTHORIZED);

  let resp4 = get("user/test").bearer_auth(token).send().unwrap();
  assert_eq!(resp4.status(), StatusCode::OK);
}

#[test]
fn unauthorized() {
  let resp = get("user/test").send().unwrap();
  assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "Expected unauthorized access in user/test.");
}
