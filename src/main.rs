mod models;
use models::user::{self, User};
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[post("/todo", data = "<user>")]
fn user_data(user: Json<user::User<'_>>) -> Json<User> {
    let new_user = user::from_json(user);
    Json(new_user)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![user_data])
}
