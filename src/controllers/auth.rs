use crate::match_pool;
use crate::models::request_models::{login_request::LoginRequest, signup_request::SignupRequest};
use crate::models::response::{Data, NetworkResponse, Response, JWT};
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
    match user_result {
        Ok(optional_user) => match optional_user {
            Some(user) => {
                if crypt::verify_password(&login_request.password, &user.password.unwrap()) {
                    let token = jwt::create_jwt(user.id);
                    match token {
                        Ok(token_str) => {
                            let login_response = LoginResponse {
                                token: token_str,
                                username: user.username,
                                email: user.email,
                            };
                            return NetworkResponse::Ok(Json(Response {
                                error_code: None,
                                message: "Login successful!",
                                data: Some(Data::Model(login_response)),
                            }));
                        }
                        Err(_err) => {
                            return NetworkResponse::InternalServerError(Json(Response {
                                error_code: Some(500),
                                message: "Service is temporarily unavailable",
                                data: None,
                            }))
                        }
                    }
                } else {
                    return NetworkResponse::BadRequest(Json(Response {
                        error_code: Some(400),
                        message: "Incorrect password!",
                        data: None,
                    }));
                }
            }
            _ => {
                return NetworkResponse::BadRequest(Json(Response {
                    error_code: Some(400),
                    message: "User does not exist!",
                    data: None,
                }))
            }
        },
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
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

    match user {
        Ok(optional_user) => match optional_user {
            Some(_user) => {
                return NetworkResponse::BadRequest(Json(Response {
                    error_code: Some(400),
                    message: "User already exists!",
                    data: None,
                }));
            }
            _ => {}
        },
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
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
        Ok(_) => {
            return NetworkResponse::Ok(Json(Response {
                error_code: None,
                message: "Signup successful!",
                data: None,
            }));
        }
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
    }
}

#[get("/hello")]
// 👇 New!
pub fn hello_token<'r>(user: JWT) -> NetworkResponse<'r, String> {
    NetworkResponse::Ok(Json(Response {
        error_code: None,
        message: "You are authenticated!",
        data: Some(Data::Model(format!(
            "Your user id is: {}",
            user.claims.subject_id
        ))),
    }))
}
