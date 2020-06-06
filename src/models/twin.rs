#[warn(unused_imports)]

use serde::{Deserialize, Serialize};
// use cdrs::frame::IntoBytes;
// use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::query_values;
use cdrs::query::QueryValues;
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Twin {
  id: Uuid,
  name: String,
  created_at: DateTime<Utc>,
  owner: Uuid
}

/// Generic element component of a Twin instance.
/// Used to define structure between other elements and to attach sources of data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Element {
  pub id: Uuid,
  pub twin: Uuid,
  pub name: String,
  pub parent: Uuid, // optional parent element
  pub created_at: DateTime<Utc>
}

impl Element {
  pub fn to_query(self) -> QueryValues {
    query_values!(
      "id" => self.id,
      "twin" => self.twin,
      "name" => self.name,
      "parent" => self.parent
      // "created_at" => self.created_at,
    )
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementRegister {
  pub name: String,
  pub parent: Uuid
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Source {
  pub id: Uuid,
  pub name: String,
  pub element: Uuid,
  pub created_at: DateTime<Utc>
  // type
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceData {
  pub source: Uuid,
  pub stamp: DateTime<Utc>,
  pub value: String,
  pub created_at: DateTime<Utc>
}

