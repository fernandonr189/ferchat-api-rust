
use crate::models::response::Data;
use crate::models::response::NetworkResponse;
use crate::models::response::Response;
use crate::models::response::JWT;
use crate::util::jwt;
use rocket::serde::json::Json;

#[post("/login")]
pub fn login<'r>() -> NetworkResponse<'r, String> {
    let response = match jwt::create_jwt(1) {
        Ok(token) => NetworkResponse::Ok(Json(Response {
            error_code: None,
            message: "Login succesful",
            data: Some(Data::Model(token)),
        })),
        Err(err) => NetworkResponse::BadRequest(Json(Response {
            error_code: Some(400),
            message: "Error loging in",
            data: Some(Data::Model(err.to_string())),
        })),
    };
    response
}

#[get("/hello")]
// 👇 New!
pub fn hello_token<'r>(user: JWT) -> NetworkResponse<'r, String> {
    NetworkResponse::Ok(Json(Response {
        error_code: None,
        message: "You are authenticated!",
        data: Some(Data::Model(format!("Your user id is: {}", user.claims.subject_id))),
    }))
}
