use serde::{Deserialize, Serialize};
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Group {
  pub id: String,
  pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupRegister {
  pub name: String
}
