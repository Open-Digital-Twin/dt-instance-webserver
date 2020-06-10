use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::QueryExecutor;

use std::sync::Arc;
use std::env;

#[cfg(test)]
pub mod models;

#[cfg(test)]
use models::app::{CurrentSession};

#[cfg(test)]
pub fn init_db_session() -> Arc<CurrentSession> {
  let db_address = env::var("DB_ADDRESS").unwrap();

  let node = NodeTcpConfigBuilder::new(
    &db_address,
    NoneAuthenticator {}
  ).build();

  let cluster_config = ClusterTcpConfig(vec![node]);

  let _session: Arc<CurrentSession> = Arc::new(
    new_session(&cluster_config, RoundRobin::new())
      .expect("session should be created")
  );

  _session
}

#[cfg(test)]
#[allow(dead_code)]
pub fn get_db_session() -> Arc<CurrentSession> {
  let _session = init_db_session();

  assert!(_session.query("USE dt;").is_ok(), "Should have set keyspace.");

  _session
}

#[cfg(test)]
#[allow(dead_code)]
fn get_api() -> String {
  let api = env::var("SERVER_ADDRESS").expect("Test API address.");

  api
}

#[cfg(test)]
#[allow(dead_code)]
fn request_create_client() -> reqwest::blocking::Client {
  reqwest::blocking::Client::new()
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_get(addr: &str) -> reqwest::blocking::RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().get(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_post(addr: &str) -> reqwest::blocking::RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().post(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_put(addr: &str) -> reqwest::blocking::RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().put(&url)
}

#[cfg(test)]
#[allow(dead_code)]
pub fn request_delete(addr: &str) -> reqwest::blocking::RequestBuilder {
  let api = get_api();
  let url = format!("http://{}/{}", api, addr);

  request_create_client().delete(&url)
}
