use std::sync::Arc;
use crate::{CurrentSession};

pub struct AppState {
  pub session: Arc<CurrentSession>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Environment {
  pub server_address: String,
  pub db_address: String,
  pub secret_key: String,
  pub twin_instance: i64
}
