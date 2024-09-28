use rocket::{request::Request, serde::json::Json};

use crate::models::{response::{NetworkResponse, Response}, user::User};

#[catch(404)]
pub fn not_found<'r>(_req: &Request) -> NetworkResponse<'r, Vec<User>> {
    NetworkResponse::NotFound(Json(Response {
        error_code: Some(404),
        message: "Not found",
        data: None,
    }))
}
#[catch(401)]
pub fn unauthorized<'r>(_req: &Request) -> NetworkResponse<'r, Vec<User>> {
    println!("{}", _req);
    NetworkResponse::Unauthorized(Json(Response {
        error_code: Some(401),
        message: "Invalid token",
        data: None,
    }))
}