use crate::util::jwt;
use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseStruct {
    pub body: ResponseBody,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Data<T> {
    Model(T),
    None,
}

#[derive(Serialize, Deserialize)]
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
