use crate::match_pool;
use crate::models::friendship::Friendship;
use crate::models::request_models::friend_acc_request::FriendAccRequest;
use crate::models::request_models::friend_req_request::FriendReqRequest;
use crate::models::response::Data;
use crate::models::response::{NetworkResponse, Response, Jwt};
use crate::models::user::{User, UserSimplified};
use crate::util::sql;
use rocket::serde::json::Json;

#[post("/request", format = "json", data = "<req>")]
pub fn send_request<'r>(user: Jwt, req: Json<FriendReqRequest>) -> NetworkResponse<'r, String> {
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
            NetworkResponse::Ok(Json(Response {
                error_code: Some(200),
                message: "Friend request sent successfully!",
                data: None,
            }))
        }
        Err(_err) => {
            println!("Error: {}", _err);
            NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
    }
}

#[post("/accept", format = "json", data = "<req>")]
pub fn accept_request<'r>(user: Jwt, req: Json<FriendAccRequest>) -> NetworkResponse<'r, String> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let user_id = user.claims.subject_id;
    let acc_request = req.into_inner();
    let target_id = acc_request.friend_id;
    let status = acc_request.accept;

    let accept_friend_request_query = format!(
        "UPDATE friends SET status = {} WHERE user_id = GREATEST({}, {}) AND friend_id = LEAST({}, {}) AND sender_id = {}",
        if status { 1 } else { 2 }, user_id, target_id, user_id, target_id, target_id);

    // Insert friend request into database
    let accept_fried_request: Result<(), mysql::Error> =
        sql::insert(&pool, &accept_friend_request_query);

    match accept_fried_request {
        Ok(_) => {
            NetworkResponse::Ok(Json(Response {
                error_code: Some(200),
                message: "Friend request accepted successfully!",
                data: None,
            }))
        }
        Err(_err) => {
            NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
    }
}

#[get("/get/<status>")]
pub fn list_friends<'r>(user: Jwt, status: &str) -> NetworkResponse<'r, Vec<UserSimplified>> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");

    let user_id = user.claims.subject_id;
    let mut sender_id = user_id;
    let mut sender_comparator = "!=";

    let status_int: i8 = match status {
        "pending" => 0,
        "accepted" => {
            sender_id = 0;
            1
        }
        "sent" => {
            sender_comparator = "=";
            0
        }
        _ => {
            return NetworkResponse::BadRequest(Json(Response {
                error_code: Some(400),
                message: "Invalid status!",
                data: None,
            }));
        }
    };

    let list_friends_query = format!(
        "SELECT u.id, u.username FROM users u JOIN friends f ON (u.id = f.friend_id OR u.id = f.user_id) WHERE (f.user_id = {} OR f.friend_id = {}) AND f.status = {} AND u.id != {} AND u.is_active = 1 AND f.sender_id {} {}",
        user_id, user_id, status_int, user_id, sender_comparator, sender_id);

    let friends_query_result: Result<Vec<UserSimplified>, mysql::Error> =
        sql::query_vec(&pool, &list_friends_query);

    match friends_query_result {
        Ok(friends) => {
            NetworkResponse::Ok(Json(Response {
                error_code: None,
                message: "Friends retrieved successfully!",
                data: Some(Data::Model(friends)),
            }))
        }
        Err(_err) => {
            NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
    }
}

#[post("/delete", format = "json", data = "<req>")]
pub fn cancel_request<'r>(
    user: Jwt,
    req: Json<FriendReqRequest>,
) -> NetworkResponse<'r, Vec<UserSimplified>> {
    let new_pool = sql::create_pool();
    let pool = match_pool!(new_pool, 500, "Could not connect to database!");
    let user_id = user.claims.subject_id;
    let target_id = req.into_inner().friend_id;

    let find_friend_request_query = format!(
        "SELECT user_id, friend_id, sender_id, status, created_at FROM friends WHERE user_id = GREATEST({}, {}) AND friend_id = LEAST({}, {}) AND sender_id = {}",
        user_id, target_id, user_id, target_id, user_id
    );

    let find_friend_request: Result<Option<Friendship>, mysql::Error> =
        sql::query_element(&pool, &find_friend_request_query);

    match find_friend_request {
        Ok(some_user) => match some_user {
            Some(_user) => {}
            _ => {
                return NetworkResponse::BadRequest(Json(Response {
                    error_code: Some(400),
                    message: "Friend request does not exist or you are not the sender",
                    data: None,
                }));
            }
        },
        Err(_err) => {
            println!("Error: {}", _err);
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }));
        }
    }

    let cancel_friend_request_query = format!(
        "DELETE FROM friends WHERE user_id = GREATEST({}, {}) AND friend_id = LEAST({}, {}) AND sender_id = {}",
        user_id, target_id, user_id, target_id, user_id);

    // Insert friend request into database
    let cancel_fried_request: Result<(), mysql::Error> =
        sql::insert(&pool, &cancel_friend_request_query);

    match cancel_fried_request {
        Ok(_) => {
            NetworkResponse::Ok(Json(Response {
                error_code: Some(200),
                message: "Friend request cancelled successfully!",
                data: None,
            }))
        }
        Err(_err) => {
            NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Service is temporarily unavailable",
                data: None,
            }))
        }
    }
}
