use crate::common::models::user::{User, Claims};
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::env;

#[derive(Debug, Deserialize)]
pub struct AuthValidator {
  pub authenticated: bool,
  pub user: User
}

impl FromRequest for AuthValidator {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  type Config = ();

  fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
    let _auth = _req.headers().get("Authorization");

    match _auth {
      Some(_) => {
        let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
        let token = _split[1].trim();
        
        let _var = env::var("SECRET_KEY").unwrap().to_string();
        let key = _var.as_bytes();

        match decode::<Claims>(
          token,
          &DecodingKey::from_secret(key),
          &Validation::new(Algorithm::HS256),
        ) {
          Ok(_token) => {
            let claim: Claims = _token.claims;
            let user: User = serde_json::from_str(&claim.sub).unwrap();

            return ok(AuthValidator {
              authenticated: true,
              user: user.clone()
            });
          },
          Err(_e) => err(ErrorUnauthorized("Invalid token.")),
        }
      }
      None => err(ErrorUnauthorized("Authentication required.")),
    }
  }
}
