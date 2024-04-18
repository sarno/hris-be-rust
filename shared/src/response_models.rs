use domain::models::Project;

use rocket::serde::{Deserialize, Serialize};
use rocket::Responder;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use chrono::Utc;
use std::env;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Post(Project),
    Posts(Vec<Project>),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub status: bool,
    pub message: String,
    pub body: ResponseBody,
}

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i64,
    pub exp: usize,
}

pub fn create_jwt(id: i64) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set."); // 👈 New!

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response {
                    status: false,
                    message: "Error ".to_string(),
                    body: ResponseBody::Message(String::from(
                        "Error validating JWT token - No token provided",
                    )),
                };
                Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()),
                ))
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response {
                            status: false,
                            message: "Error ".to_string(),
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Expired Token"
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(
                                serde_json::to_string(&response).unwrap(),
                            ),
                        ))
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response {
                            status: false,
                            message: "Error ".to_string(),
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Invalid Token"
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(
                                serde_json::to_string(&response).unwrap(),
                            ),
                        ))
                    }
                    _ => {
                        let response = Response {
                            status: false,
                            message: "Error ".to_string(),
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - {}",
                                err
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(
                                serde_json::to_string(&response).unwrap(),
                            ),
                        ))
                    }
                },
            },
        }
    }
}
