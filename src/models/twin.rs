use serde::{Deserialize, Serialize};
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use chrono::prelude::*;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct Twin {
  id: i64,
  name: String
}

/// Generic element component of a Twin instance.
/// Used to define structure between other elements and to attach sources of data.
#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Element {
  pub id: i64,
  pub twin: i64,
  pub name: String,
  pub parent: i64 // optional parent element
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementRegister {
  pub name: String,
  pub parent: i64
}

pub struct Source {
  pub id: i64,
  pub name: String,
  pub element: i64
  // type
}

pub struct SourceData {
  pub source: i64,
  pub timestamp: DateTime<Utc>,
  pub value: String,
}


