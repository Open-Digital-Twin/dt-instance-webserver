use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use std::sync::Arc;
use std::env;

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

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

pub fn get_db_session() -> Arc<CurrentSession> {
  let _session = init_db_session();

  assert!(_session.query("USE dt;").is_ok(), "Should have set keyspace.");

  _session
}

#[test]
fn _0_0_create_db() {
  let _session = init_db_session();

  assert!(_session.query("DROP KEYSPACE IF EXISTS dt;").is_ok(), "Should have deleted keyspace.");

  assert!(_session.query("
    CREATE KEYSPACE IF NOT EXISTS dt WITH REPLICATION = {
      'class' : 'SimpleStrategy',
      'replication_factor' : 1 
    };
  ").is_ok(), "Should have created keyspace.");

  assert!(_session.query("USE dt;").is_ok(), "Should have set keyspace.");

  assert!(_session.query("
    CREATE TABLE IF NOT EXISTS user (
      id UUID,
      email text,
      name text,
      password text,
      PRIMARY KEY (email)
    );
  ").is_ok(), "Should have created user table");

  assert!(_session.query("
    CREATE TABLE IF NOT EXISTS twin (
      id UUID,
      name text,
      created_at timestamp,
      owner UUID,
      PRIMARY KEY (id)
    );
  ").is_ok(), "Should have created twin table");

  assert!(_session.query("
    CREATE TABLE IF NOT EXISTS element (
      id UUID,
      twin UUID,
      name text,
      created_at timestamp,
      parent UUID,
      PRIMARY KEY (id)
    );
  ").is_ok(), "Should have created element table");

  assert!(_session.query("
    CREATE TABLE IF NOT EXISTS source (
      id UUID,
      name text,
      element UUID,
      created_at timestamp,
      PRIMARY KEY (id)
    );
  ").is_ok(), "Should have created source table");

  assert!(_session.query("
    CREATE TABLE IF NOT EXISTS source_data (
      source UUID,
      stamp timestamp,
      value text,
      created_at timestamp,
      PRIMARY KEY (source, stamp)
    );
  ").is_ok(), "Should have created source data table");

  // Spec elements, received from DT Master
  assert!(_session.query("
    INSERT INTO user (email, id, name, password) VALUES (
      'example@example.com',
      296375b2-6ba4-4e22-ad0f-ac963d4e9171,
      'Example User',
      '$argon2i$v=19$m=4096,t=3,p=1$NFVVZElWUER0a3VPNUM0RUIzQWNSYWNFWmZyamxYSzM$kQdDj7+5oagkvKSyU+TWiczRT3Y4m5A/6YY8XDEf5Gs'
    );
  ").is_ok(), "Should have created spec user");

  assert!(_session.query("
    INSERT INTO twin (id, created_at, name, owner) VALUES (
      38162cb0-e585-43d7-b55d-5c240b2bfb7c,
      toTimestamp(now()),
      'Twin Instance',
      296375b2-6ba4-4e22-ad0f-ac963d4e9171
    );
  ").is_ok(), "Should have created spec twin");
}
