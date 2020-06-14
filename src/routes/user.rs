use cdrs::query_values;
use cdrs::query::*;
use cdrs::frame::TryFromRow;

use crate::common::models::user::{User, UserLogin, Claims, Register};
use crate::common::models::app::{CurrentSession, Environment};
use crate::common::models::response::{LoginResponse, Response, DataResponse};
use crate::middlewares::auth::AuthValidator;
use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use argon2::{self, Config};
use rand::{ thread_rng, Rng };
use rand::distributions::Alphanumeric;

use log::{info, error};

use actix_web::{get, post, web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};

#[post("/login")]
async fn login(session: web::Data<Arc<CurrentSession>>, _env: web::Data<Environment>, user_login: web::Json<UserLogin>) -> HttpResponse {
  let _usr = get_user_from_email(session.clone(), user_login.email.clone());

  match _usr {
    Err(e) => {
      error!("{}", e);
      return HttpResponse::Ok().json(Response {
        message: format!("Invalid email {}.", user_login.email.to_string()),
        status: true
      });
    },
    Ok(_user) => {
      match authenticate(user_login.clone(), _user.clone(), &_env) {
        Err(_) => {
          return HttpResponse::Ok().json(Response {
            status: false,
            message: "Invalid password informed.".to_string(),
          })
        },
        Ok(token) => {
          return HttpResponse::Ok().json(LoginResponse {
            status: true,
            token,
            message: "You have successfully logged in.".to_string(),
          });
        }
      }
    }
  }
}

fn authenticate(_login: UserLogin, user: User, _env: &Environment) -> Result<String, String> {
  if verify_hash(&_login.password, &user.password) {
    let mut _date: DateTime<Utc>;
    
    if !_login.remember_me {
      _date = Utc::now() + Duration::hours(1);
    } else {
      _date = Utc::now() + Duration::days(365);
    }
    
    let claim = Claims {
      sub: serde_json::to_string(&user).unwrap(),
      exp: _date.timestamp() as usize,
    };
    
    let token = encode(
      &Header::default(),
      &claim,
      &EncodingKey::from_secret(
        _env.secret_key.as_bytes()
      ),
    ).unwrap();

    return Ok(token.to_string());
  } else {
    return Err("Invalid password input".to_string());
  }
}

fn generate_hash(password: &String) -> String {
  let config = Config::default();
  let salt = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .collect::<String>();

  let hash = argon2::hash_encoded(
    &password.as_bytes(),
    &salt.as_bytes(),
    &config
  ).unwrap();

  return hash.to_string();
}

fn verify_hash(password: &String, hash: &String) -> bool {
  return argon2::verify_encoded(
    &hash.to_string(),
    &password.as_bytes()
  ).unwrap();
}

#[post("/register")]
async fn register(session: web::Data<Arc<CurrentSession>>, _env: web::Data<Environment>, user: web::Json<Register>) -> HttpResponse {
  let _usr = get_user_from_email(session.clone(), user.email.clone());

  match _usr {
    Ok(_u) => HttpResponse::Ok().json(Response {
      message: format!("User {} already exists.", _u.email.to_string()),
      status: false
    }),
    Err(_) => {
      session.query_with_values(
        "INSERT INTO user (email, id, name, password) VALUES (?, ?, ?, ?)",
        query_values!(
          user.email.to_string(),
          uuid::Uuid::new_v4(),
          user.name.to_string(),
          generate_hash(&user.password).to_string()
        )
      ).expect("Inserted new user");

      info!("New user {}.", user.email);
      let new_user = get_user_from_email(session.clone(), user.email.clone()).unwrap();

      // TODO: Handle creation error;
      // TODO: Remove password hash from answer;

      HttpResponse::Ok().json(DataResponse {
        message: format!("Success in creating user {}.", user.email.to_string()),
        status: true,
        data: new_user
      })
    }
  }
}

pub fn get_user_from_email(session: web::Data<Arc<CurrentSession>>, email: String) -> Result<User, String> {
  let rows = session.query_with_values(
    "SELECT * FROM user WHERE email = ? ALLOW FILTERING",
    query_values!(email)
  )
    .expect("select user with email")
    .get_body().unwrap()
    .into_rows().unwrap();

  if !rows.is_empty() {
    let usr = match User::try_from_row(rows[0].clone()) {
      Ok(_model) => _model,
      Err(_) => return Err("Could not convert rows to User model.".to_string())
    };

    return Ok(usr);
  }
  return Err("No user with selected email".to_string());
}

// #[post("/userInformations")]
// async fn user_informations(_req: HttpRequest) -> HttpResponse {
//   let _auth = _req.headers().get("Authorization");
//   let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//   let token = _split[1].trim();
//   let _connection: Connection = Connection {};
//   let _repository: UserRepository = UserRepository {
//     connection: _connection.init(),
//   };
//   match _repository.user_informations(token) {
//     Ok(result) => HttpResponse::Ok().json(result.unwrap()),
//     Err(err) => HttpResponse::Ok().json(err),
//   }
// }

// #[get("/userInformations")]
// async fn user_informations_get(_req: HttpRequest) -> HttpResponse {
//   let _auth = _req.headers().get("Authorization");
//   let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//   let token = _split[1].trim();
//   let _connection: Connection = Connection {};
//   let _repository: UserRepository = UserRepository {
//     connection: _connection.init(),
//   };
//   match _repository.user_informations(token) {
//     Ok(result) => HttpResponse::Ok().json(result.unwrap()),
//     Err(err) => HttpResponse::Ok().json(err),
//   }
// }

#[get("/test")]
async fn temp(_auth: AuthValidator) -> HttpResponse {
  println!("{}", _auth.user.email);

  HttpResponse::Ok().json(Response {
    status: true,
    message: "User logged in.".to_string()
  })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(login);
  cfg.service(register);
  cfg.service(temp);
}
