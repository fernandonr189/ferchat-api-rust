use crate::util::jwt;
use jsonwebtoken::errors::ErrorKind;
use rocket::http::Header;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Responder;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Responder, Debug)]
pub enum NetworkResponse<'r, T> {
    #[response(status = 200)]
    Ok(Json<Response<'r, T>>),
    #[response(status = 400)]
    BadRequest(Json<Response<'r, T>>),
    #[response(status = 401)]
    Unauthorized(Json<Response<'r, T>>),
    #[response(status = 404)]
    NotFound(Json<Response<'r, T>>),
    #[response(status = 500)]
    InternalServerError(Json<Response<'r, T>>),
}

pub struct CorsResponder;

impl<'r> Responder<'r, 'static> for CorsResponder {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            .header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ))
            .header(Header::new("Access-Control-Allow-Origin", "*"))
            .header(Header::new("Access-Control-Allow-Headers", "*"))
            .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .status(Status::Ok)
            .ok()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum Data<T> {
    Model(T),
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response<'a, T> {
    pub error_code: Option<i32>,
    pub message: &'a str,
    pub data: Option<Data<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub subject_id: i32,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Jwt {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Jwt {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, String> {
        fn is_valid(key: &str) -> Result<Claims, ErrorKind> {
            jwt::decode_jwt(String::from(key))
        }

        let key = match req.headers().get_one("authorization") {
            None => {
                return Outcome::Error((Status::Unauthorized, String::from("Missing auth header")))
            }
            Some(key) => key,
        };

        let error = match is_valid(key) {
            Ok(claims) => return Outcome::Success(Jwt { claims }),
            Err(err) => err,
        };

        match error {
            ErrorKind::ExpiredSignature => {
                Outcome::Error((Status::Unauthorized, String::from("Token has expired")))
            }
            ErrorKind::InvalidToken => {
                Outcome::Error((Status::Unauthorized, String::from("Token is invalid")))
            }
            ErrorKind::InvalidAlgorithm => {
                Outcome::Error((Status::Unauthorized, String::from("Invalid algorithm")))
            }
            _ => Outcome::Error((
                Status::Unauthorized,
                String::from("Unknown error decoding token"),
            )),
        }
    }
}
