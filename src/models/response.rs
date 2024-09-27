use crate::util::jwt;
use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Responder, Debug)]
pub enum NetworkResponse<'r, T> {
    #[response(status = 200)]
    Ok(Json<Response<'r, T>>),
    #[response(status = 201)]
    Created(Json<Response<'r, T>>),
    #[response(status = 400)]
    BadRequest(Json<Response<'r, T>>),
    #[response(status = 401)]
    Unauthorized(Json<Response<'r, T>>),
    #[response(status = 404)]
    NotFound(Json<Response<'r, T>>),
    #[response(status = 409)]
    Conflict(Json<Response<'r, T>>),
    #[response(status = 500)]
    InternalServerError(Json<Response<'r, T>>),
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

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse<'r, String>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse<'r, String>> {
        fn is_valid(key: &str) -> Result<Claims, ErrorKind> {
            Ok(jwt::decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Error((
                Status::Unauthorized,
                NetworkResponse::Unauthorized(Json(Response {
                    error_code: Some(401),
                    message: "Missing Auth header",
                    data: None,
                })),
            )),
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized(Json(Response {
                            error_code: Some(401),
                            message: "Token has expired",
                            data: None,
                        })),
                    )),
                    jsonwebtoken::errors::ErrorKind::InvalidToken => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized(Json(Response {
                            error_code: Some(401),
                            message: "Token is invalid",
                            data: None,
                        })),
                    )),
                    _ => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized(Json(Response {
                            error_code: Some(401),
                            message: "Unknown error validating token",
                            data: None,
                        })),
                    )),
                },
            },
        }
    }
}
