use crate::match_pool;
use crate::models::request_models::{login_request::LoginRequest, signup_request::SignupRequest};
use crate::models::response::{Data, Jwt, NetworkResponse, Response};
use crate::models::response_models::login_response::LoginResponse;
use crate::models::user::User;
use crate::util::{crypt, jwt, sql};
use rocket::serde::json::Json;

#[post("/login", format = "json", data = "<req>")]
pub fn login<'r>(req: Json<LoginRequest>) -> NetworkResponse<'r, LoginResponse> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let login_request = req.into_inner();

    let query = format!(
        "SELECT id, username, password, email, is_active FROM users WHERE email = '{}' AND is_active = 1",
        login_request.email
    );

    let user_result: Result<Option<User>, mysql::Error> = sql::query_element(&pool, &query);
    let usr_opt = match user_result {
        Ok(optional_user) => optional_user,
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
    };
    let user = match usr_opt {
        Some(user) => user,
        _ => {
            return NetworkResponse::BadRequest(Json(Response {
                error_code: Some(400),
                message: "User does not exist!",
                data: None,
            }))
        }
    };
    if crypt::verify_password(&login_request.password, &user.password.unwrap()) {
        let token_str = match jwt::create_jwt(user.id) {
            Ok(token_str) => token_str,
            Err(_err) => {
                return NetworkResponse::InternalServerError(Json(Response {
                    error_code: Some(500),
                    message: "Service is temporarily unavailable",
                    data: None,
                }))
            }
        };
        let login_response = LoginResponse {
            token: token_str,
            username: user.username,
            email: user.email,
        };
        NetworkResponse::Ok(Json(Response {
            error_code: None,
            message: "Login successful!",
            data: Some(Data::Model(login_response)),
        }))
    } else {
        NetworkResponse::BadRequest(Json(Response {
            error_code: Some(400),
            message: "Incorrect password!",
            data: None,
        }))
    }
}

#[post("/signup", format = "json", data = "<req>")]
pub fn signup<'r>(req: Json<SignupRequest>) -> NetworkResponse<'r, String> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let signup_request = req.into_inner();

    let user: Result<Option<User>, mysql::Error> = sql::query_element(
        &pool,
        &format!(
            "SELECT id, username, password, email, is_active FROM users WHERE username = '{}' AND is_active = 1",
            signup_request.username
        ),
    );

    let user_opt = match user {
        Ok(optional_user) => optional_user,
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
    };
    if let Some(_user) = user_opt {
        return NetworkResponse::BadRequest(Json(Response {
            error_code: Some(400),
            message: "User already exists!",
            data: None,
        }));
    }

    let query = format!(
        "INSERT INTO users(
        username,
        password,
        email,
        is_active
        ) VALUES (
        '{}',
        '{}',
        '{}',
        1
        )",
        signup_request.username,
        crypt::hash_password(&signup_request.password),
        signup_request.email
    );

    let signup_result = sql::insert(&pool, &query);
    match signup_result {
        Ok(_) => NetworkResponse::Ok(Json(Response {
            error_code: None,
            message: "Signup successful!",
            data: None,
        })),
        Err(_err) => NetworkResponse::InternalServerError(Json(Response {
            error_code: Some(500),
            message: "Service is temporarily unavailable",
            data: None,
        })),
    }
}

#[get("/hello")]
// 👇 New!
pub fn hello_token<'r>(user: Jwt) -> NetworkResponse<'r, String> {
    NetworkResponse::Ok(Json(Response {
        error_code: None,
        message: "You are authenticated!",
        data: Some(Data::Model(format!(
            "Your user id is: {}",
            user.claims.subject_id
        ))),
    }))
}
