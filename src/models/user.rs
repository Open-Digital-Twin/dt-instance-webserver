use serde::{Deserialize, Serialize};
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct User {
  pub email: String,
  pub id: Uuid,
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
  pub sub: String, // Serialized User
  pub exp: usize
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Register {
  pub name: String,
  pub email: String,
  pub password: String
}
