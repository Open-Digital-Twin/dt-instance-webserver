#![allow(dead_code)]
extern crate argon2;
extern crate rand;

extern crate envy;
extern crate env_logger;

use log::{info};

#[macro_use]
extern crate serde_derive;

use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use std::sync::Arc;
use std::env;

#[macro_use]
extern crate cdrs_helpers_derive;

use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};

use dotenv::dotenv;

mod middlewares;

mod common;
use common::models::app::*;
use common::db::get_db_session;

mod routes;

fn strip_comment<'a>(input: &'a str, markers: &[char]) -> &'a str {
  input
    .find(markers)
    .map(|idx| &input[..idx])
    .unwrap_or(input)
    .trim()
}

fn start_db_session(addr: String) -> Arc<CurrentSession> {
  info!("Starting db session at {} for worker", addr);

  let node = NodeTcpConfigBuilder::new(&addr, NoneAuthenticator {}).build();
  let cluster_config = ClusterTcpConfig(vec![node]);

  let _session: Arc<CurrentSession> = Arc::new(
    new_session(&cluster_config, RoundRobin::new())
      .expect("session should be created")
  );

  match _session.query("USE dt;") {
    Err(_) => info!("Database not initiated."),
    Ok(_) => info!("Set db as dt.")
  }
  
  _session
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init();

  HttpServer::new(move || {
    App::new()
      .data(envy::from_env::<Environment>().unwrap())
      .data(get_db_session().clone())
      .wrap(middleware::Compress::new(ContentEncoding::Br))
      .wrap(middleware::Logger::default())
      .service(web::scope("/user").configure(routes::user::init_routes))
      .service(web::scope("/element").configure(routes::element::init_routes))
      .service(web::scope("/source").configure(routes::source::init_routes))
      .service(web::scope("/twin").configure(routes::twin::init_routes))
      // .configure(routes_config)
  })
  .bind(env::var("SERVER_ADDRESS").unwrap())?
  .workers(1)
  .run()
  .await
}

