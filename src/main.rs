mod models;
mod util;
use models::response::Response;
use models::user::from_json;
use models::user::User;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use util::sql::*;

#[macro_use]
extern crate rocket;

#[get("/get")]
fn get_users<'r>() -> status::Custom<Json<Vec<User>>> {
    let pool = create_pool();
    let users = query(&pool, "SELECT * FROM users").expect("Could not get users");
    status::Custom(Status::Ok, Json(users))
}

#[post("/signup", data = "<user>")]
fn create_user<'r>(user: Json<User>) -> status::Custom<Json<Response<'r>>> {
    let new_user = from_json(user);
    let pool = create_pool();
    let is_inserted = insert(&pool, &new_user).unwrap();
    if is_inserted {
        status::Custom(
            Status::Ok,
            Json(Response {
                error_code: None,
                message: "User created!",
            }),
        )
    } else {
        status::Custom(
            Status::BadRequest,
            Json(Response {
                error_code: Some(400),
                message: "User not created!",
            }),
        )
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/users", routes![create_user])
        .mount("/users", routes![get_users])
}
