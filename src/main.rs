mod models;
mod util;
use models::response::Response;
use models::user::from_json;
use models::user::User;
use rocket::serde::json::Json;
use util::sql::*;

#[macro_use]
extern crate rocket;

#[post("/signup", data = "<user>")]
fn create_user<'r>(user: Json<User>) -> Json<Response<'r>> {
    let new_user = from_json(user);
    let pool = create_pool();
    let is_inserted = insert(&pool, &new_user).unwrap();
    if is_inserted {
        Json(Response {
            status_code: 200,
            message: "User created!",
        })
    } else {
        Json(Response {
            status_code: 400,
            message: "User not created!",
        })
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/users", routes![create_user])
}
