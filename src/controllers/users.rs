use crate::match_pool;
use crate::match_response;
use crate::models::response::{Data, Response};
use crate::models::user::from_json;
use crate::models::user::User;
use crate::util::sql::{create_pool, insert, query};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/get")]
pub fn get_users<'r>() -> status::Custom<Json<Response<'r, Vec<User>>>> {
    let new_pool = create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let users_result = query(&pool, "SELECT id, username, email, is_active FROM users");
    match_response!(
        users_result,
        users,
        "Users retrieved!",
        Some(Data::Model(users)),
        "Error getting users!",
        Status::InternalServerError
    )
}

#[post("/signup", data = "<user>")]
pub fn create_user<'r>(user: Json<User>) -> status::Custom<Json<Response<'r, String>>> {
    let new_user = from_json(user);
    let new_pool = create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let query_str = format!(
        "SELECT id, username, email, is_active FROM users WHERE username = '{}' OR email = '{}'",
        new_user.username, new_user.email
    );
    let users_result = query::<User>(&pool, &query_str);
    match users_result {
        Ok(users) => {
            if users.len() > 0 {
                return status::Custom(
                    Status::BadRequest,
                    Json(Response {
                        error_code: None,
                        message: "User already exists",
                        data: None,
                    }),
                );
            }
        }
        Err(_e) => {
            return status::Custom(
                Status::InternalServerError,
                Json(Response {
                    error_code: None,
                    message: "There was a problem procesing the request",
                    data: None,
                }),
            );
        }
    }

    let inserted = insert(&pool, &new_user);
    match_response!(
        inserted,
        _is_inserted,
        "User created!",
        None,
        "User not created",
        Status::BadRequest
    )
}
