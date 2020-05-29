use serde::{Deserialize, Serialize};
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct User {
  pub email: String,
  pub id: String,
  pub name: String,
  pub password: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLogin {
  pub email: String,
  pub password: String,
  #[serde(default)]
  pub remember_me: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub sub: String,
  pub exp: usize
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Register {
  pub name: String,
  pub email: String,
  pub password: String
}
