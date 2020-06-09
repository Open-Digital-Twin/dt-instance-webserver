mod init;

use reqwest::StatusCode;
use serde_json::json;

use crate::cdrs::query::QueryExecutor;

#[macro_use]
extern crate cdrs;

#[test]
fn create_user() {
  let session = init::get_db_session();
  
  // Create user
  let user_1 = json!({
    "email": "example_1@example.com",
    "name": "Example user",
    "password": "example_password"
  });
  session.query(format!("DELETE FROM user WHERE email='{}'", user_1["email"].as_str().unwrap()).to_string()).unwrap();
  
  let resp_1 = init::request_post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);

  let resp_1_body: init::Response = resp_1.json().unwrap();

  assert_eq!(resp_1_body.message, format!("Success in creating user {}.", user_1["email"].as_str().unwrap()));
  assert_eq!(resp_1_body.status, true);

  // Create user with same email should fail
  let resp_2 = init::request_post("user/register")
    .json(&user_1).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_body: init::Response = resp_2.json().unwrap();

  assert_eq!(resp_2_body.message, format!("User {} already exists.", user_1["email"].as_str().unwrap()));
  assert_eq!(resp_2_body.status, false);
}

#[test]
/// Register new user, then tests a route that requires authentication.
fn login() {
  let session = init::get_db_session();

  let register = json!({
    "email": "example_2@example.com",
    "name": "Example user",
    "password": "example_password"
  });
  session.query(format!("DELETE FROM user WHERE email='{}'", register["email"].as_str().unwrap()).to_string()).unwrap();

  let resp_1 = init::request_post("user/register")
    .json(&register).send().unwrap();

  assert_eq!(resp_1.status(), StatusCode::OK);

  let login = json!({
    "email": register["email"].as_str().unwrap(),
    "password": register["password"].as_str().unwrap()
  });

  let resp_2 = init::request_post("user/login")
    .json(&login).send().unwrap();

  assert_eq!(resp_2.status(), StatusCode::OK);

  let resp_2_json: init::LoginResponse = resp_2.json().unwrap();
  let token = resp_2_json.token;
  assert!(resp_2_json.status);

  // Test authentication with logged in user
  let resp3 = init::request_get("user/test").send().unwrap();
  assert_eq!(resp3.status(), StatusCode::UNAUTHORIZED);

  let resp4 = init::request_get("user/test").bearer_auth(token).send().unwrap();
  assert_eq!(resp4.status(), StatusCode::OK);
}

#[test]
fn unauthorized() {
  let resp = init::request_get("user/test").send().unwrap();
  assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "Expected unauthorized access in user/test.");
}
