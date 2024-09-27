use crate::match_pool;
use crate::models::response::Data;
use crate::models::response::NetworkResponse;
use crate::models::response::Response;
use crate::models::user::User;
use crate::util::sql::{create_pool, query};
use rocket::serde::json::Json;

#[get("/get")]
pub fn get_users<'r>() -> NetworkResponse<'r, Vec<User>> {
    let new_pool = create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let users_result: Result<Vec<User>, mysql::Error> = query(&pool, "SELECT id, username, email, is_active FROM users");
    match users_result {
        Ok(users) => NetworkResponse::Ok(Json(Response {
            error_code: None,
            message: "Users retreived!",
            data: Some(Data::Model(users))
        })),
        Err(_e) => NetworkResponse::InternalServerError(Json(Response {
            error_code: Some(500),
            message: "Unknown error getting users",
            data: None,
        }))
    }
}