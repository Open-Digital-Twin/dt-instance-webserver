use serde::{Deserialize, Serialize};
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct Twin {
  id: String,
  name: String,
  created_at: DateTime<Utc>,
  owner: String
}

/// Generic element component of a Twin instance.
/// Used to define structure between other elements and to attach sources of data.
#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Element {
  pub id: String,
  pub twin: String,
  pub name: String,
  pub parent: String, // optional parent element
  pub created_at: DateTime<Utc>
}

impl Element {
  fn to_query(self) -> QueryValues {
    query_values!(
      "id" => self.id,
      "twin" => self.twin,
      "name" => self.name,
      "parent" => self.parent,
      "created_at" => self.created_at
    )
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementRegister {
  pub name: String,
  pub parent: String
}

#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Source {
  pub id: String,
  pub name: String,
  pub element: String,
  pub created_at: DateTime<Utc>
  // type
}

#[derive(Serialize, Deserialize, Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct SourceData {
  pub source: String,
  pub stamp: DateTime<Utc>,
  pub value: String,
  pub created_at: DateTime<Utc>
}

