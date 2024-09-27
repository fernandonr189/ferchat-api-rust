use crate::models::response::NetworkResponse;
use crate::models::response::Response;
use crate::models::response::Data;
use crate::models::response::JWT;
use rocket::serde::json::Json;
use crate::util::jwt;

#[post("/login")]
pub fn login<'r>() -> NetworkResponse<'r, String> {
    let response = match jwt::create_jwt(1) {
        Ok(token) => 
            NetworkResponse::Ok(Json(Response {
                error_code: None,
                message: "Login succesful",
                data: Some(Data::Model(token)),
            })),
        Err(err) => 
            NetworkResponse::BadRequest(Json(Response {
                error_code: Some(400),
                message: "Error loging in",
                data: Some(Data::Model(err.to_string())),
            }))
    };
    response
}

#[post("/hello")]
pub fn hello_token<'r>(key: Result<JWT, NetworkResponse<'r, String>>) -> Result<String, NetworkResponse<'r, String>> {
    let key = key?;
    todo!()
}