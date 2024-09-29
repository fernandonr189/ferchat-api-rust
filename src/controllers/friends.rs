use crate::match_pool;
use crate::models::request_models::friend_req_request::FriendReqRequest;
use crate::models::response::Data;
use crate::models::response::{NetworkResponse, Response, JWT};
use crate::models::user::{User, UserSimplified};
use crate::util::sql;
use rocket::serde::json::Json;

#[post("/request", format = "json", data = "<req>")]
pub fn send_request<'r>(user: JWT, req: Json<FriendReqRequest>) -> NetworkResponse<'r, String> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let user_id = user.claims.subject_id;
    let target_id = req.into_inner().friend_id;

    if user_id == target_id {
        return NetworkResponse::BadRequest(Json(Response {
            error_code: Some(400),
            message: "You cannot send a friend request to yourself!",
            data: None,
        }));
    }

    let friend_exists_query = format!(
        "SELECT id, username, password, email, is_active FROM users WHERE id = '{}' AND is_active = 1",
        target_id);

    let user_exists: Result<Option<User>, mysql::Error> =
        sql::query_element(&pool, &friend_exists_query);

    // If target user does not exist or database error occurs throw an error
    match user_exists {
        Ok(some_user) => match some_user {
            Some(_user) => {}
            _ => {
                return NetworkResponse::BadRequest(Json(Response {
                    error_code: Some(400),
                    message: "User does not exist!",
                    data: None,
                }));
            }
        },
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }));
        }
    }

    let insert_friend_request_query = format!(
        "INSERT INTO friends (user_id, friend_id, sender_id) VALUES (GREATEST({}, {}), LEAST({}, {}), {})",
        user_id, target_id, user_id, target_id, user_id);

    // Insert friend request into database
    let insert_fried_request: Result<(), mysql::Error> =
        sql::insert(&pool, &insert_friend_request_query);

    match insert_fried_request {
        Ok(_) => {
            return NetworkResponse::Ok(Json(Response {
                error_code: Some(200),
                message: "Friend request sent successfully!",
                data: None,
            }));
        }
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }));
        }
    }
}

#[post("/accept", format = "json", data = "<req>")]
pub fn accept_request<'r>(user: JWT, req: Json<FriendReqRequest>) -> NetworkResponse<'r, String> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let user_id = user.claims.subject_id;
    let target_id = req.into_inner().friend_id;

    let accept_friend_request_query = format!(
        "UPDATE friends SET status = 1 WHERE user_id = GREATEST({}, {}) AND friend_id = LEAST({}, {}) AND sender_id = {}",
        user_id, target_id, user_id, target_id, target_id);

    // Insert friend request into database
    let accept_fried_request: Result<(), mysql::Error> =
        sql::insert(&pool, &accept_friend_request_query);

    match accept_fried_request {
        Ok(_) => {
            return NetworkResponse::Ok(Json(Response {
                error_code: Some(200),
                message: "Friend request accepted successfully!",
                data: None,
            }));
        }
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }));
        }
    }
}

#[get("/get")]
pub fn list_friends<'r>(user: JWT) -> NetworkResponse<'r, Vec<UserSimplified>> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");

    let user_id = user.claims.subject_id;

    let list_friends_query = format!(
        "SELECT u.id, u.username FROM users u JOIN friends f ON (u.id = f.friend_id OR u.id = f.user_id) WHERE (f.user_id = {} OR f.friend_id = {}) AND f.status = 1",
        user_id, user_id);

    let friends_query_result: Result<Vec<UserSimplified>, mysql::Error> =
        sql::query_vec(&pool, &list_friends_query);

    match friends_query_result {
        Ok(friends) => {
            return NetworkResponse::Ok(Json(Response {
                error_code: None,
                message: "Friends retrieved successfully!",
                data: Some(Data::Model(friends)),
            }));
        }
        Err(_err) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }));
        }
    }
}
