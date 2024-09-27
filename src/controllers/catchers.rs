use rocket::{request::Request, serde::json::Json};

use crate::models::{response::{NetworkResponse, Response}, user::User};

#[catch(404)]
pub fn not_found<'r>(req: &Request) -> NetworkResponse<'r, Vec<User>> {
    NetworkResponse::NotFound(Json(Response {
        error_code: Some(404),
        message: "Not found",
        data: None,
    }))
}