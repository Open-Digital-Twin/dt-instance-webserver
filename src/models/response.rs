use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
  pub message: String,
  pub status: bool,
  pub token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  pub message: String,
  pub status: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataResponse<T> {
  pub data: T,
  pub message: String,
  pub status: bool
}
