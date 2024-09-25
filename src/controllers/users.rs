use crate::models::response::{Data, Response};
use crate::models::user::from_json;
use crate::models::user::User;
use crate::util::sql::{create_pool, insert, query};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

macro_rules! match_pool {
    ($a: expr, $b: expr, $c: expr) => {
        match $a {
            Ok(exp) => exp,
            Err(_e) => {
                return status::Custom(
                Status::InternalServerError,
                Json(Response {
                    error_code: Some($b),
                    message: $c,
                    data: None
                })
            )
            }
        }
    }
}

#[get("/get")]
pub fn get_users<'r>() -> status::Custom<Json<Response<'r, Vec<User>>>> {
    let new_pool = create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let users_result = query(&pool, "SELECT id, username, email, is_active FROM users");
    match users_result {
        Ok(users) => status::Custom(
            Status::Ok,
            Json(Response {
                error_code: None,
                message: "Users retrieved!",
                data: Some(Data::Model(users)),
            }),
        ),
        Err(_e) => status::Custom(
            Status::InternalServerError,
            Json(Response {
                error_code: Some(500),
                message: "Error getting users!",
                data: None,
            }),
        ),
    }
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
    match inserted {
        Ok(_is_inserted) => {
            return status::Custom(
                Status::Ok,
                Json(Response {
                    error_code: None,
                    message: "User created!",
                    data: None,
                }),
            )
        }
        Err(_e) => {
            return status::Custom(
                Status::BadRequest,
                Json(Response {
                    error_code: None,
                    message: "User not created",
                    data: None,
                }),
            )
        }
    };
}