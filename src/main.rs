mod controllers;
mod models;
mod util;
use controllers::users::create_user;
use controllers::users::get_users;
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/users", routes![create_user])
        .mount("/users", routes![get_users])
}
